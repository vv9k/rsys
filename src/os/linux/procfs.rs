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
    Mounts,
    NetDev,
    PidStat(i32),
    Proc,
}
impl ProcPath {
    pub(crate) fn path(self) -> String {
        match self {
            ProcPath::Hostname => "/proc/sys/kernel/hostname".to_string(),
            ProcPath::DomainName => "/proc/sys/kernel/domainname".to_string(),
            ProcPath::CpuInfo => "/proc/cpuinfo".to_string(),
            ProcPath::MemInfo => "/proc/meminfo".to_string(),
            ProcPath::Uptime => "/proc/uptime".to_string(),
            ProcPath::KernelRelease => "/proc/sys/kernel/osrelease".to_string(),
            ProcPath::Mounts => "/proc/mounts".to_string(),
            ProcPath::NetDev => "/proc/net/dev".to_string(),
            ProcPath::PidStat(n) => format!("/proc/{}/stat", n),
            ProcPath::Proc => "/proc".to_string(),
        }
    }

    pub(crate) fn read(self) -> Result<String, Error> {
        let path = self.path();
        fs::read_to_string(&path).map_err(|e| Error::FileReadError(path, e.to_string()))
    }
}
