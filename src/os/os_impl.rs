use crate::Result;

/// Common api
pub(crate) trait OsImpl {
    fn hostname(&self) -> Result<String>;
    fn uptime(&self) -> Result<u64>;
    fn arch(&self) -> Result<String>;
    fn cpu(&self) -> Result<String>;
    fn cpu_clock(&self) -> Result<f32>;
    fn cpu_cores(&self) -> Result<u16>;
    fn logical_cores(&self) -> Result<u16>;
    fn memory_total(&self) -> Result<usize>;
    fn memory_free(&self) -> Result<usize>;
    fn swap_total(&self) -> Result<usize>;
    fn swap_free(&self) -> Result<usize>;
    fn default_iface(&self) -> Result<String>;
    fn ipv4(&self, iface: &str) -> Result<String>;
    fn ipv6(&self, iface: &str) -> Result<String>;
    fn mac(&self, iface: &str) -> Result<String>;
    fn interfaces(&self) -> Result<Vec<String>>;
    fn domainname(&self) -> Result<String>;
}
