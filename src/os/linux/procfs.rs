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
    PidStat(u64),
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
            ProcPath::Mounts => "/proc/mounts",
            ProcPath::NetDev => "/proc/net/dev",
            ProcPath::PidStat(_) => "/proc",
        }
    }

    pub(crate) fn read(self) -> Result<String, Error> {
        let path = match self.clone() {
            ProcPath::PidStat(n) => format!("{}/{}/stat", self.path(), n),
            _ => self.path().to_string(),
        };
        fs::read_to_string(&path).map_err(|e| Error::FileReadError(path, e.to_string()))
    }
}
