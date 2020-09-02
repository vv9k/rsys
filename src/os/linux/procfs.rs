use super::Error;
use std::fs;

#[derive(Copy, Clone)]
pub(crate) enum ProcPath {
    Hostname,
    DomainName,
    CpuInfo,
    MemInfo,
    Uptime,
    KernelRelease,
}
impl ProcPath {
    fn path(self) -> &'static str {
        match self {
            ProcPath::Hostname => "/proc/sys/kernel/hostname",
            ProcPath::DomainName => "/proc/sys/kernel/domainname",
            ProcPath::CpuInfo => "/proc/cpuinfo",
            ProcPath::MemInfo => "/proc/meminfo",
            ProcPath::Uptime => "/proc/uptime",
            ProcPath::KernelRelease => "/proc/sys/kernel/osrelease",
        }
    }

    pub(crate) fn read(self) -> Result<String, Error> {
        let p = self.clone().path();
        fs::read_to_string(p).map_err(|e| Error::FileReadError(p.to_string(), e.to_string()))
    }
}
