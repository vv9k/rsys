//! Linux specific api
#![cfg(target_os = "linux")]

#[cfg(test)]
pub(crate) mod mocks;

pub mod cpu;
pub mod mem;
pub mod mounts;
pub mod net;
mod os_impl_ext;
pub mod ps;
pub mod storage;
mod sysinfo;
mod sysproc;

pub use crate::os::unix::{arch, domainname, hostname, kernel_release};
pub use sysinfo::{sysinfo, SysInfo};
pub(crate) use sysproc::{SysFs, SysPath};
pub(crate) use {
    cpu::{cpu, cpu_clock, cpu_cores, logical_cores},
    mem::{memory_free, memory_total, swap_free, swap_total},
    net::{default_iface, interfaces, ipv4, ipv6, mac},
    os_impl_ext::OsImplExt,
};

use crate::os::OsImpl;
use crate::Result;

/// Returns uptime of host machine in seconds
pub fn uptime() -> Result<u64> {
    Ok(sysinfo()?.uptime().as_secs())
}

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
