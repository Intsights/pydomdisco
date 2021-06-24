use pyo3::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;
use tokio::task;
use std::sync::Arc;
use trust_dns_client::client::{AsyncClient, ClientHandle};
use trust_dns_client::rr::{DNSClass, Name, RecordType};
use trust_dns_client::udp::UdpClientStream;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;

pub struct Discoverer {
    rt: Runtime,
    resolvers: HashMap<String, Vec<AsyncClient>>,
}

impl Discoverer {
    pub fn new(
        tld_to_nameservers_ips: HashMap<String, Vec<String>>,
        nameserver_timeout: Duration,
    ) -> Discoverer {
        let rt = Runtime::new().unwrap();

        let resolvers = rt.block_on(
            async move {
                let mut resolvers = HashMap::<String, Vec<AsyncClient>>::new();
                for (tld, nameservers_a) in tld_to_nameservers_ips.iter() {
                    for nameserver_a in nameservers_a.iter() {
                        let stream = UdpClientStream::<UdpSocket>::with_timeout(
                            format!("{}:53", nameserver_a).parse().unwrap(),
                            nameserver_timeout,
                        );
                        let (client, bg) = AsyncClient::connect(stream).await.unwrap();
                        task::spawn(bg);

                        if resolvers.contains_key(tld) {
                            resolvers.get_mut(tld).unwrap().push(client);
                        } else {
                            resolvers.insert(tld.to_string(), vec![client]);
                        }
                    }
                }

                resolvers
            }
        );

        Discoverer { rt, resolvers }
    }

    async fn is_domain_registered(
        domain: String,
        mut resolvers: Vec<AsyncClient>,
    ) -> bool {
        let domain_rr = Name::from_str(&domain).unwrap();

        for resolver in resolvers.iter_mut() {
            let future = resolver.query(
                domain_rr.clone(),
                DNSClass::IN,
                RecordType::NS,
            );
            match future.await {
                Ok(response) => {
                    for answer in response.answers() {
                        if answer.rr_type() == RecordType::NS && answer.name() == &domain_rr {
                            return true;
                        }
                    }
                    for additional in response.additionals() {
                        if additional.rr_type() == RecordType::NS && additional.name() == &domain_rr {
                            return true;
                        }
                    }
                    for name_server in response.name_servers() {
                        if name_server.rr_type() == RecordType::NS && name_server.name() == &domain_rr {
                            return true;
                        }
                    }

                    return false;
                }
                Err(_error) => {},
            }
        }

        false
    }

    pub fn discover(
        &self,
        py: &Python,
        records: &[String],
        chunk_size: usize,
    ) -> Result<Vec<String>, PyErr> {
        self.rt.block_on(
            async move {
                let mut active_domains = Vec::<String>::with_capacity(10000);
                for records_chunk in records.chunks(chunk_size) {
                    let mut join_handles = Vec::with_capacity(
                        self.resolvers.len() * records_chunk.len()
                    );
                    for record in records_chunk {
                        for (tld, tld_resolvers) in self.resolvers.iter() {
                            let domain = format!("{}.{}", record, tld);
                            let join_handle = task::spawn(
                                Discoverer::is_domain_registered(
                                    domain.clone(),
                                    tld_resolvers.to_vec(),
                                )
                            );
                            join_handles.push((domain, join_handle));
                        }
                    }

                    for (domain, join_handle) in join_handles {
                        py.check_signals()?;

                        if let Ok(domain_is_registered) = join_handle.await {
                            if domain_is_registered {
                                active_domains.push(domain);
                            }
                        }
                    }
                }

                Ok(active_domains)
            }
        )
    }

    pub fn generate_tld_to_nameservers_ips(
        py: &Python,
        tlds: Vec<String>,
    ) -> Result<HashMap<String, Vec<String>>, PyErr> {
        Runtime::new().unwrap().block_on(
            async move {
                let mut tld_to_nameservers_ips = HashMap::<String, Vec<String>>::new();

                let resolver = Arc::new(
                    TokioAsyncResolver::tokio(
                        ResolverConfig::default(),
                        trust_dns_resolver::config::ResolverOpts {
                            timeout: Duration::new(2, 0),
                            attempts: 1,
                            ..Default::default()
                        },
                    ).unwrap()
                );

                let mut futures = Vec::new();
                for tld in tlds.into_iter() {
                    let resolver = resolver.clone();
                    let future = task::spawn(
                        async move {
                            let mut ip_addresses: Vec<String> = Vec::new();

                            if let Ok(response) = resolver.ns_lookup(tld.clone()).await {
                                for nameserver in response {
                                    if let Ok(response) = resolver.lookup_ip(nameserver).await {
                                        response
                                            .iter()
                                            .filter(|x| x.is_ipv4())
                                            .for_each(|x| ip_addresses.push(x.to_string()));
                                    }
                                }
                            }

                            (tld, ip_addresses)
                        }
                    );
                    futures.push(future);
                }

                for future in futures {
                    py.check_signals()?;

                    if let Ok((tld, nameservers_a)) = future.await {
                        tld_to_nameservers_ips.insert(tld, nameservers_a);
                    }
                }

                Ok(tld_to_nameservers_ips)
            }
        )
    }
}
