//! MacOS specific api
mod public;

use super::{run, Error, OsImpl};
use std::process::Command;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) use public::*;

const SYSCTL_CPU: &str = "machdep.cpu.brand_string";
const SYSCTL_HOSTNAME: &str = "kern.hostname";
const SYSCTL_BOOTTIME: &str = "kern.boottime";
const SYSCTL_MODEL: &str = "hw.model";
const SYSCTL_MEMSIZE: &str = "hw.memsize";
const CPU_FREQUENCY: &str = "hw.cpufrequency";
const CPU_CORES: &str = "hw.physicalcpu";
const LOGICAL_CORES: &str = "hw.logicalcpu";
const VM_PAGESIZE: &str = "vm.pagesize";

const INTERFACE: &str = "interface: ";
const INTERFACE_LEN: usize = INTERFACE.len();
const SYSCTL_BOOTTIME_LEN: usize = "{ sec = ".len();
const PAGES_ACTIVE: &str = "Pages active:";
const PAGES_INACTIVE: &str = "Pages inactive:";

//################################################################################
// INTERNAL

fn sysctl(property: &str) -> Result<String, Error> {
    run(Command::new("sysctl").arg("-n").arg(property))
}

fn vm_pagesize() -> Result<u32, Error> {
    Ok(sysctl(VM_PAGESIZE)?
        .parse::<u32>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

//################################################################################
// UNIQUE

/// Returns a model of host machine.
pub fn model() -> Result<String, Error> {
    sysctl(SYSCTL_MODEL)
}
