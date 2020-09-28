#![allow(dead_code)]

use crate::{Error, Result};
use std::{fmt::Display, fs, path::PathBuf, str::FromStr};

#[derive(Clone)]
pub(crate) enum SysPath<'p> {
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
    SysBlockDevSize(&'p str),
    SysBlockDevStat(&'p str),
    SysBlockDevModel(&'p str),
    SysBlockDevVendor(&'p str),
    SysBlockDevState(&'p str),
    SysBlockDevDev(&'p str),
    SysBlockDev(&'p str),
    SysClassBlock(&'p str),
    SysDevMapperName(&'p str),
    SysDevMapperUuid(&'p str),
    SysDevicesSystemCpu(),
    SysDevicesSystemCpuCore(u32),
    Dev(&'p str),
    Custom(PathBuf),
}
impl<'p> SysPath<'p> {
    pub(crate) fn path(self) -> PathBuf {
        use SysPath::*;
        let s = match self {
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
            SysClassBlock(d) => format!("/sys/class/block/{}", d),
            SysDevMapperName(d) => format!("/sys/block/{}/dm/name", d),
            SysDevMapperUuid(d) => format!("/sys/block/{}/dm/uuid", d),
            SysDevicesSystemCpu() => "/sys/devices/system/cpu".to_string(),
            SysDevicesSystemCpuCore(d) => format!("/sys/devices/system/cpu/cpu{}", d),
            Dev(d) => format!("/dev/{}", d),
            Custom(p) => p.to_string_lossy().to_string(),
        };
        PathBuf::from(s)
    }

    pub(crate) fn read(self) -> Result<String> {
        let path = self.path();
        fs::read_to_string(&path)
            .map_err(|e| Error::FileReadError(path.as_path().to_string_lossy().to_string(), e.to_string()))
    }

    pub(crate) fn read_as<T: FromStr>(self) -> Result<T>
    where
        <T as FromStr>::Err: Display,
    {
        let path = self.path();
        let data = fs::read_to_string(&path)
            .map_err(|e| Error::FileReadError(path.as_path().to_string_lossy().to_string(), e.to_string()))?;

        T::from_str(data.trim()).map_err(|e| Error::InvalidInputError(data, e.to_string()))
    }

    pub(crate) fn read_dir(self) -> Result<fs::ReadDir> {
        let path = self.path();
        fs::read_dir(&path)
            .map_err(|e| Error::FileReadError(path.as_path().to_string_lossy().to_string(), e.to_string()))
    }

    pub(crate) fn extend(self, p: &[&str]) -> Self {
        let mut path = self.path();
        for elem in p {
            path.push(elem);
        }
        SysPath::Custom(path)
    }

    pub(crate) fn read_path(self, p: &[&str]) -> Result<String> {
        let mut path = self.path();
        for elem in p {
            path.push(elem);
        }
        fs::read_to_string(&path).map_err(|e| Error::FileReadError(path.to_string_lossy().to_string(), e.to_string()))
    }
}
