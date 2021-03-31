//! Linux specific api
#![cfg(target_os = "linux")]

#[cfg(test)]
pub(crate) mod mocks;

pub mod cpu;
#[cfg(feature = "display")]
mod display;
pub mod mem;
pub mod misc;
pub mod net;
mod os_impl_ext;
pub mod ps;
pub mod storage;
mod sysinfo;
mod sysproc;

use std::ffi::CStr;

use super::OsImpl;
use crate::{Error, Result};
use libc::{c_char, size_t};
use nix::{errno::Errno, sys::utsname, unistd};

pub use sysinfo::{sysinfo, SysInfo};
pub(crate) use {
    cpu::{cpu, cpu_clock, cpu_cores, logical_cores},
    mem::{memory_free, memory_total, swap_free, swap_total},
    net::{default_iface, interfaces, ipv4, ipv6, mac},
    os_impl_ext::OsImplExt,
};

pub(crate) use sysproc::SysPath;

/// Returns a hostname.
pub fn hostname() -> Result<String> {
    let mut buf = [0u8; 64];
    Ok(unistd::gethostname(&mut buf)?.to_string_lossy().to_string())
}

/// Returns uptime of host machine in seconds
fn uptime() -> Result<u64> {
    Ok(sysinfo()?.uptime().as_secs())
}

/// Returns the processor architecture
pub fn arch() -> Result<String> {
    Ok(utsname::uname().machine().to_string())
}

/// Returns a domainname read from /proc/sys/kernel/domainname
pub fn domainname() -> Result<String> {
    const BUF_LEN: usize = 64; // Acording to manual entry of getdomainname this is the limit
                               // of length for the domain name
    let mut buf = [0u8; BUF_LEN];
    let ptr = buf.as_mut_ptr() as *mut c_char;
    let len = BUF_LEN as size_t;

    let res = unsafe { libc::getdomainname(ptr, len) };
    Errno::result(res)
        .map(|_| {
            unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) }
                .to_string_lossy()
                .to_string()
        })
        .map_err(Error::from)
}

/// Returns a kernel release of host os.
pub fn kernel_release() -> Result<String> {
    Ok(utsname::uname().release().to_string())
}

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
