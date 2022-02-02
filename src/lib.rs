mod discovery;

use pyo3::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

/// Discoverer class
///
/// input:
///     None
///
/// example:
///     pydomdisco.Discoverer()
#[pyclass]
struct Discoverer {
    discoverer: discovery::Discoverer,
}

#[pymethods]
impl Discoverer {
    #[new]
    fn new(
        tld_to_nameservers_ips: HashMap<String, Vec<String>>,
        nameserver_timeout: f64,
    ) -> Self {
        let discoverer = discovery::Discoverer::new(
            tld_to_nameservers_ips,
            Duration::from_secs_f64(nameserver_timeout),
        );

        Discoverer { discoverer }
    }

    /// Discover registered dns records under the requested name
    ///
    /// input:
    ///     names: list[str] -> The names to look for, without the trailing dot.
    ///     chunk_size: int -> The number of names to process concurrently.
    ///
    /// returns:
    ///     list[str] -> list of registered domains
    ///
    /// example:
    ///     registered_domains = mass_resolver.discover(
    ///         names=[
    ///             'google',
    ///             'facebook',
    ///         ],
    ///     )
    fn discover(
        &mut self,
        py: Python,
        names: Vec<String>,
        chunk_size: usize,
    ) -> PyResult<Vec<String>> {
        self.discoverer.discover(
            &py,
            names.as_slice(),
            chunk_size,
        )
    }

    /// Iterating the tlds and find their corresponding NS records' IP addresses
    ///
    /// input:
    ///     tlds: list[str] -> List of tlds
    ///
    /// returns:
    ///     dict[str, list[str]] -> List of tlds to their corresponding NS records' IP addresses
    ///
    /// example:
    ///     registered_domains = pydomdisco.Discoverer.generate_tld_to_nameservers_ips(
    ///         tlds=['com', 'net', 'org'],
    ///     )
    #[staticmethod]
    fn generate_tld_to_nameservers_ips(
        py: Python,
        tlds: Vec<String>,
    ) -> PyResult<HashMap<String, Vec<String>>> {
        discovery::Discoverer::generate_tld_to_nameservers_ips(&py, tlds)
    }
}

/// PyDomDisco takes a record and discovers
/// active domains under the world's known TLDs.
#[pymodule]
fn pydomdisco(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Discoverer>()?;

    Ok(())
}
