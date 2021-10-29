use crate::Result;

/// Common api
pub(crate) trait OsImpl {
    fn hostname(&self) -> Result<String>;
    fn domain_name(&self) -> Result<String>;
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
}
