use super::Error;
use std::fs;

#[derive(Clone)]
pub(crate) enum SysPath {
    ProcHostname,
    ProcDomainName,
    ProcCpuInfo,
    ProcMemInfo,
    ProcUptime,
    ProcKernelRelease,
    ProcMounts,
    ProcNetDev,
    ProcPidStat(i32),
    Proc,
    SysBlockDevSize(String),
    SysBlockDevStat(String),
    SysBlockDevModel(String),
    SysBlockDevVendor(String),
    SysBlockDevState(String),
    SysBlockDevDev(String),
    SysBlockDev(String),
    SysDevMapperName(String),
    SysDevMapperUuid(String),
}
impl SysPath {
    pub(crate) fn path(self) -> String {
        use SysPath::*;
        match self {
            ProcHostname => "/proc/sys/kernel/hostname".to_string(),
            ProcDomainName => "/proc/sys/kernel/domainname".to_string(),
            ProcCpuInfo => "/proc/cpuinfo".to_string(),
            ProcMemInfo => "/proc/meminfo".to_string(),
            ProcUptime => "/proc/uptime".to_string(),
            ProcKernelRelease => "/proc/sys/kernel/osrelease".to_string(),
            ProcMounts => "/proc/mounts".to_string(),
            ProcNetDev => "/proc/net/dev".to_string(),
            ProcPidStat(n) => format!("/proc/{}/stat", n),
            Proc => "/proc".to_string(),
            SysBlockDevSize(d) => format!("/sys/block/{}/size", d),
            SysBlockDevStat(d) => format!("/sys/block/{}/stat", d),
            SysBlockDevModel(d) => format!("/sys/block/{}/device/model", d),
            SysBlockDevVendor(d) => format!("/sys/block/{}/device/vendor", d),
            SysBlockDevState(d) => format!("/sys/block/{}/device/state", d),
            SysBlockDevDev(d) => format!("/sys/block/{}/dev", d),
            SysBlockDev(d) => format!("/sys/block/{}", d),
            SysDevMapperName(d) => format!("/sys/block/{}/dm/name", d),
            SysDevMapperUuid(d) => format!("/sys/block/{}/dm/uuid", d),
        }
    }

    pub(crate) fn read(self) -> Result<String, Error> {
        let path = self.path();
        fs::read_to_string(&path).map_err(|e| Error::FileReadError(path, e.to_string()))
    }
}
