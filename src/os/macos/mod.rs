//! MacOS specific api
mod internal;
mod os_impl_ext;
mod public;

use super::{run, OsImpl};
use crate::{Error, Result};
use std::{
    process::Command,
    str,
    time::{SystemTime, UNIX_EPOCH},
};

pub(crate) use internal::*;
pub use os_impl_ext::OsImplExt;
pub use public::*;

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

#[derive(Default, OsImpl)]
pub(crate) struct Macos {}
impl OsImplExt for Macos {}
