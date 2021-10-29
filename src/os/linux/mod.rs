//! Linux specific api
#![cfg(target_os = "linux")]

#[cfg(test)]
pub(crate) mod mocks;

pub mod cpu;
pub mod mem;
pub mod mounts;
mod os_impl_ext;
pub mod ps;
mod sysinfo;
mod sysproc;

pub use crate::os::unix::{arch, domain_name, hostname, kernel_release};
pub use sysinfo::{sysinfo, SysInfo};
pub(crate) use sysproc::{SysFs, SysPath};
pub(crate) use {
    cpu::{cpu, cpu_clock, cpu_cores, logical_cores},
    mem::{memory_free, memory_total, swap_free, swap_total},
    os_impl_ext::OsImplExt,
};

use crate::os::OsImpl;
use crate::Result;

/// Returns uptime of host machine in seconds
pub fn uptime() -> Result<u64> {
    Ok(sysinfo()?.uptime().as_secs())
}

#[derive(Default)]
pub(crate) struct Linux {}

impl Linux {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OsImpl for Linux {
    fn hostname(&self) -> Result<String> {
        hostname()
    }

    fn domain_name(&self) -> Result<String> {
        domain_name()
    }

    fn uptime(&self) -> Result<u64> {
        uptime()
    }

    fn arch(&self) -> Result<String> {
        arch()
    }

    fn cpu(&self) -> Result<String> {
        cpu()
    }

    fn cpu_clock(&self) -> Result<f32> {
        cpu_clock()
    }

    fn cpu_cores(&self) -> Result<u16> {
        cpu_cores()
    }

    fn logical_cores(&self) -> Result<u16> {
        logical_cores()
    }

    fn memory_total(&self) -> Result<usize> {
        memory_total()
    }

    fn memory_free(&self) -> Result<usize> {
        memory_free()
    }

    fn swap_total(&self) -> Result<usize> {
        swap_total()
    }

    fn swap_free(&self) -> Result<usize> {
        swap_free()
    }
}
