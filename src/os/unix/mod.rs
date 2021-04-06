#![cfg_attr(target_os = "macos", allow(dead_code))]
use crate::{Error, Result};

#[cfg(not(target_os = "macos"))]
use libc::size_t;
#[cfg(target_os = "macos")]
use std::os::raw::c_int;

use libc::c_char;
use nix::{errno::Errno, sys::utsname, unistd};
use std::ffi::CStr;

/// Returns a hostname.
pub fn hostname() -> Result<String> {
    let mut buf = [0u8; 64];
    Ok(unistd::gethostname(&mut buf)?.to_string_lossy().to_string())
}

/// Returns the processor architecture
pub fn arch() -> Result<String> {
    Ok(utsname::uname().machine().to_string())
}

/// Returns a domainname by calling getdomainname syscall
pub fn domainname() -> Result<String> {
    const BUF_LEN: usize = 64; // Acording to manual entry of getdomainname this is the limit
                               // of length for the domain name
    let mut buf = [0u8; BUF_LEN];
    let ptr = buf.as_mut_ptr() as *mut c_char;

    #[cfg(target_os = "macos")]
    let len = BUF_LEN as c_int;
    #[cfg(not(target_os = "macos"))]
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
