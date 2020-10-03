//! Linux specific api
#![cfg(target_os = "linux")]

#[cfg(test)]
pub(crate) mod mocks;
#[cfg(test)]
use mocks::UPTIME;

pub mod cpu;
#[cfg(feature = "display")]
mod display;
pub mod mem;
pub mod misc;
pub mod net;
mod os_impl_ext;
pub mod ps;
pub mod storage;
mod sysproc;

use super::{run, OsImpl};
use crate::{Error, Result};
use std::process::Command;

pub(crate) use {
    cpu::{cpu, cpu_clock, cpu_cores, logical_cores},
    mem::{memory_free, memory_total, swap_free, swap_total},
    net::{default_iface, interfaces, ipv4, ipv6, mac},
    os_impl_ext::OsImplExt,
};

pub(crate) use sysproc::SysPath;

/// Returns a hostname.
pub fn hostname() -> Result<String> {
    Ok(SysPath::ProcHostname.read()?.trim().to_string())
}

/// Internal implementation of parsing uptime from /proc/uptime
fn _uptime(out: &str) -> Result<u64> {
    Ok(out
        .split_ascii_whitespace()
        .take(1)
        .collect::<String>()
        .parse::<f64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))? as u64)
}

/// Returns current uptime.
pub fn uptime() -> Result<u64> {
    _uptime(&SysPath::ProcUptime.read()?)
}

/// Returns the processor architecture
pub fn arch() -> Result<String> {
    run(Command::new("uname").arg("-m"))
}

/// Returns a domainname read from /proc/sys/kernel/domainname
pub fn domainname() -> Result<String> {
    Ok(SysPath::ProcDomainName.read()?.trim().to_string())
}

/// Returns a kernel version of host os.
pub fn kernel_version() -> Result<String> {
    SysPath::ProcKernelRelease.read().map(|s| s.trim().to_string())
}

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gets_uptime() {
        assert_eq!(_uptime(UPTIME), Ok(5771))
    }
}
