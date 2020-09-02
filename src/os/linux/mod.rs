//! Linux specific api
#![cfg(target_os = "linux")]

mod internal;
mod public;

use super::{run, Error, OsImpl};
use std::fs;
use std::process::Command;

pub use public::{
    arch, cpu, cpu_clock, cpu_cores, default_iface, domainname, hostname, interfaces, ipv4, ipv6, kernel_version,
    logical_cores, mac, memory, swap, uptime,
};

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
const TOTAL_MEM: &str = "MemTotal:";
const TOTAL_SWAP: &str = "SwapTotal:";

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
