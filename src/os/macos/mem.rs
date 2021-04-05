use crate::macos::system::*;
use crate::{Error, Result};

use std::mem;
use sysctl::CtlValue;

//################################################################################
// Public
//################################################################################

pub fn memory_total() -> Result<usize> {
    match sysctl(SYSCTL_MEMSIZE)? {
        CtlValue::S64(num) => Ok(num as usize),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}

pub fn memory_free() -> Result<usize> {
    // this returns a 32bit value so on systems with > 2GB of memory it will
    // show incorrect values. More on this here https://discussions.apple.com/thread/1775836
    match sysctl(SYSCTL_USERMEM)? {
        CtlValue::Int(num) => Ok(num as usize),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}

pub fn swap_total() -> Result<usize> {
    get_swap().map(|swap| swap.total as usize)
}

pub fn swap_free() -> Result<usize> {
    get_swap().map(|swap| swap.free as usize)
}

//################################################################################
// Internal
//################################################################################

#[derive(Clone, Default)]
#[repr(C)]
struct SwapUsage {
    total: libc::c_long,
    used: libc::c_long,
    free: libc::c_long,
}

fn get_swap() -> Result<SwapUsage> {
    match sysctl(SYSCTL_VM_SWAPUSAGE)? {
        CtlValue::Struct(mut val) => {
            let val_ptr = val.as_mut_ptr();
            let swapusage_ptr = val_ptr as *mut SwapUsage;
            let swapusage_ref = unsafe { &mut *swapusage_ptr };
            Ok(mem::take(swapusage_ref))
        }
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}
