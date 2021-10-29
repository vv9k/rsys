use crate::macos::system::*;
use crate::{Error, Result};

use std::time::{SystemTime, UNIX_EPOCH};
use sysctl::CtlValue;

//################################################################################
// Public
//################################################################################

pub fn hostname() -> Result<String> {
    match sysctl(SYSCTL_HOSTNAME)? {
        CtlValue::String(hostname) => Ok(hostname),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}

pub fn uptime() -> Result<u64> {
    let boot = match sysctl(SYSCTL_BOOTTIME)? {
        CtlValue::Struct(time) => {
            let time_ptr = time.as_ptr();
            let uptime_ptr = time_ptr as *const Uptime;
            let uptime_ref = unsafe { &*uptime_ptr };
            Ok(uptime_ref.sec as u64)
        }
        val => Err(Error::UnexpectedSysctlValue(val)),
    }?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::TimeError(e.to_string()))?
        .as_secs();

    Ok(now - boot)
}

pub fn domain_name() -> Result<String> {
    match sysctl(SYSCTL_DOMAINNAME)? {
        CtlValue::String(cpu) => Ok(cpu),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}

//################################################################################
// UNIQUE
//################################################################################

/// Returns a model of host machine.
pub fn model() -> Result<String> {
    match sysctl(SYSCTL_MODEL)? {
        CtlValue::String(cpu) => Ok(cpu),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}

//################################################################################
// Internal
//################################################################################

#[repr(C)]
struct Uptime {
    sec: libc::time_t,
    usec: libc::time_t,
}
