//! Linux specific api
#![cfg(target_os = "linux")]

mod internal;
mod public;

use super::{run, Error, OsImpl};
use std::fs;
use std::process::Command;

pub use public::*;

pub(crate) use internal::*;

const HOSTNAME: &str = "/proc/sys/kernel/hostname";
const DOMAINNAME: &str = "/proc/sys/kernel/domainname";
const CPU: &str = "/proc/cpuinfo";
const MEM: &str = "/proc/meminfo";
const UPTIME: &str = "/proc/uptime";
const KERNEL: &str = "/proc/sys/kernel/osrelease";

const MODEL_NAME: &str = "model name";
const CPU_CORES: &str = "cpu cores";
const SIBLINGS: &str = "siblings";
const CPU_CLOCK: &str = "cpu MHz";
const MEM_TOTAL: &str = "MemTotal:";
const MEM_FREE: &str = "MemAvailable:";
const SWAP_TOTAL: &str = "SwapTotal:";
const SWAP_FREE: &str = "SwapFree:";

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
