use super::Error;

/// Common api
pub(crate) trait OsImpl {
    fn hostname(&self) -> Result<String, Error>;
    fn uptime(&self) -> Result<u64, Error>;
    fn arch(&self) -> Result<String, Error>;
    fn cpu(&self) -> Result<String, Error>;
    fn cpu_clock(&self) -> Result<f32, Error>;
    fn cpu_cores(&self) -> Result<u16, Error>;
    fn logical_cores(&self) -> Result<u16, Error>;
    fn memory_total(&self) -> Result<usize, Error>;
    fn memory_free(&self) -> Result<usize, Error>;
    fn swap_total(&self) -> Result<usize, Error>;
    fn swap_free(&self) -> Result<usize, Error>;
    fn default_iface(&self) -> Result<String, Error>;
    fn ipv4(&self, iface: &str) -> Result<String, Error>;
    fn ipv6(&self, iface: &str) -> Result<String, Error>;
    fn mac(&self, iface: &str) -> Result<String, Error>;
    fn interfaces(&self) -> Result<Vec<String>, Error>;
    fn domainname(&self) -> Result<String, Error>;
}
