#![cfg(target_os = "linux")]

mod public;

use super::{run, Error, OsImpl};
use std::fs;
use std::process::Command;

pub(crate) use public::Linux;

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

//################################################################################
// INTERNAL

fn ip(iface: &str) -> Result<serde_json::Value, Error> {
    let mut _ip = Command::new("ip");
    let mut cmd = if iface == "" {
        _ip.arg("-j").arg("address").arg("show")
    } else {
        _ip.arg("-j").arg("address").arg("show").arg(&iface)
    };
    Ok(serde_json::from_str::<serde_json::Value>(&run(&mut cmd)?)
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

//################################################################################
// UNIQUE

/// Returns a kernel version of host os.
pub fn kernel_version() -> Result<String, Error> {
    Ok(fs::read_to_string(KERNEL).map_err(|e| Error::FileReadError(UPTIME.to_string(), e.to_string()))?)
}
