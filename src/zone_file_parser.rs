use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::fmt;

enum RecordClass {
    IN,
    CH,
    HS,
}

#[derive(Debug, Clone)]
struct RecordClassParseError;

impl fmt::Display for RecordClassParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid record type")
    }
}

impl FromStr for RecordClass {
    type Err = RecordClassParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "in" => Ok(RecordClass::IN),
            "ch" => Ok(RecordClass::CH),
            "hs" => Ok(RecordClass::HS),
            _ => Err(RecordClassParseError),
        }
    }
}

#[derive(Debug, PartialEq)]
enum RecordType {
    A,
    NS,
    MX,
}

#[derive(Debug, Clone)]
struct RecordTypeParseError;

impl fmt::Display for RecordTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid record type")
    }
}

impl FromStr for RecordType {
    type Err = RecordTypeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" => Ok(RecordType::A),
            "ns" => Ok(RecordType::NS),
            "mx" => Ok(RecordType::MX),
            _ => Err(RecordTypeParseError),
        }
    }
}

struct ResourceRecord<T> {
    host_label: String,
    ttl: u32,
    record_class: RecordClass,
    record_type: RecordType,
    record_data: T,
}

pub struct ZoneFileParser {
    pub ns_records: HashMap<String, Vec<String>>,
    pub a_records: HashMap<String, Vec<Ipv4Addr>>,
}

impl ZoneFileParser {
    pub fn new_from_text(text: &str) -> Self {
        let mut ns_records = HashMap::<String, Vec<String>>::new();
        let mut a_records = HashMap::<String, Vec<Ipv4Addr>>::new();
        for line in text.split('\n') {
            if line.contains("\tNS\t") {
                let resource_record = ZoneFileParser::parse_ns_record_line("", line).unwrap();
                if resource_record.host_label.is_empty() {
                    continue;
                }

                match ns_records.get_mut(&resource_record.host_label) {
                    Some(ns) => {
                        ns.push(resource_record.record_data);
                    }
                    None => {
                        ns_records.insert(
                            resource_record.host_label,
                            vec![resource_record.record_data]
                        );
                    }
                };
            } else if line.contains("\tA\t") {
                let resource_record = ZoneFileParser::parse_a_record_line("", line).unwrap();
                match a_records.get_mut(&resource_record.host_label) {
                    Some(ips) => {
                        ips.push(resource_record.record_data);
                    }
                    None => {
                        a_records.insert(
                            resource_record.host_label,
                            vec![resource_record.record_data]
                        );
                    }
                };
            }
        }

        ZoneFileParser {
            ns_records,
            a_records,
        }
    }

    pub fn get_tld_to_nameservers_a(
        self: &Self,
    ) -> HashMap<String, Vec<Ipv4Addr>> {
        let mut tld_to_nameservers_a = HashMap::<String, Vec<Ipv4Addr>>::new();
        for (tld, nameservers_ns) in self.ns_records.iter() {
            for nameserver_ns in nameservers_ns.iter() {
                if !self.a_records.contains_key(nameserver_ns) {
                    continue;
                }

                for nameserver_a in self.a_records[nameserver_ns].iter() {
                    if tld_to_nameservers_a.contains_key(tld) {
                        tld_to_nameservers_a
                            .get_mut(tld)
                            .unwrap()
                            .push(nameserver_a.to_owned());
                    } else {
                        tld_to_nameservers_a
                            .insert(tld.to_owned(), vec![nameserver_a.to_owned()]);
                    }
                }
            }
        }

        tld_to_nameservers_a
    }

    fn parse_a_record_line(
        origin: &str,
        line: &str,
    ) -> Result<ResourceRecord<Ipv4Addr>, String> {
        let line_components: Vec<&str> = line.split_whitespace().collect();

        if line_components.len() != 5 {
            return Err(format!("Line number of parts is invalid. Expected 5, found {}", line_components.len()));
        }

        let mut host_label = line_components[0].to_ascii_lowercase();
        if !host_label.ends_with('.') {
            host_label = format!("{}.{}", host_label, origin);
        }
        host_label = host_label.strip_suffix('.').unwrap().to_string();


        let ttl = line_components[1].parse::<u32>().unwrap();
        let record_class = line_components[2].parse::<RecordClass>().unwrap();
        let record_type = line_components[3].parse::<RecordType>().unwrap();
        if record_type != RecordType::A {
            return Err(format!("Invalid record type. Expected 'A', found {:?}", record_type));
        }
        let record_data = line_components[4].parse::<Ipv4Addr>().unwrap();

        Ok(
            ResourceRecord {
                host_label,
                ttl,
                record_class,
                record_type,
                record_data,
            }
        )
    }

    fn parse_ns_record_line(
        origin: &str,
        line: &str,
    ) -> Result<ResourceRecord<String>, String> {
        let line_components: Vec<&str> = line.split_whitespace().collect();

        if line_components.len() != 5 {
            return Err(format!("Line number of parts is invalid. Expected 5, found {}", line_components.len()));
        }

        let mut host_label = line_components[0].to_ascii_lowercase();
        if !host_label.ends_with('.') {
            host_label = format!("{}.{}", host_label, origin);
        }
        host_label = host_label.strip_suffix('.').unwrap().to_string();

        let ttl = line_components[1].parse::<u32>().unwrap();
        let record_class = line_components[2].parse::<RecordClass>().unwrap();
        let record_type = line_components[3].parse::<RecordType>().unwrap();
        if record_type != RecordType::NS {
            return Err(format!("Invalid record type. Expected 'NS', found {:?}", record_type));
        }

        let mut record_data = line_components[4].parse::<String>().unwrap();
        if !record_data.ends_with('.') {
            record_data = format!("{}.{}", record_data, origin);
        }
        record_data = record_data.strip_suffix('.').unwrap().to_string();

        Ok(
            ResourceRecord {
                host_label,
                ttl,
                record_class,
                record_type,
                record_data,
            }
        )
    }
}
