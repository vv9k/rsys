#![cfg(target_os = "linux")]
use super::{run, Error};
use std::fs;
use std::process::Command;

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

pub(crate) fn _default_iface() -> Result<String, Error> {
    let mut cmd = Command::new("route");
    Ok(run(&mut cmd)?
        .split('\n')
        .filter(|l| l.starts_with("default"))
        .collect::<String>()
        .split_ascii_whitespace()
        .last()
        .ok_or_else(|| Error::CommandParseError("output of route command was invalid".to_string()))?
        .to_string())
}

pub(crate) fn _hostname() -> Result<String, Error> {
    Ok(fs::read_to_string(HOSTNAME)
        .map_err(|e| Error::FileReadError(HOSTNAME.to_string(), e.to_string()))?
        .trim()
        .to_string())
}

pub(crate) fn _domainname() -> Result<String, Error> {
    Ok(fs::read_to_string(DOMAINNAME)
        .map_err(|e| Error::FileReadError(DOMAINNAME.to_string(), e.to_string()))?
        .trim()
        .to_string())
}

pub(crate) fn _ipv4(iface: &str) -> Result<String, Error> {
    let out = ip(&iface)?;
    let ip = &out[0]["addr_info"][0]["local"];
    if ip.is_string() {
        // It's ok to unwrap here because we know it's a string
        return Ok(ip.as_str().map(|s| s.to_string()).unwrap());
    }

    Err(Error::CommandParseError(format!(
        "ip address '{:?}' was not a string",
        ip
    )))
}

pub(crate) fn _mac(iface: &str) -> Result<String, Error> {
    let out = ip(&iface)?;
    let mac = &out[0]["address"];
    if mac.is_string() {
        // It's ok to unwrap here because we know it's a string
        return Ok(mac.as_str().map(|s| s.to_string()).unwrap());
    }

    Err(Error::CommandParseError(format!(
        "mac address '{:?}' was not a string",
        mac
    )))
}

pub(crate) fn _interfaces() -> Result<Vec<String>, Error> {
    let out = ip("")?;
    if !out.is_array() {
        return Err(Error::CommandParseError("invalid 'ip' command output".to_string()));
    }

    // It's ok to unwrap here because we check that out is an array and all non-string values are filtered out
    Ok(out
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["ifname"].is_string())
        .map(|v| v["ifname"].as_str().unwrap().to_string())
        .collect())
}

pub(crate) fn _ipv6(_iface: &str) -> Result<String, Error> {
    todo!()
}

pub(crate) fn _cpu() -> Result<String, Error> {
    Ok(fs::read_to_string(CPU)
        .map_err(|e| Error::FileReadError(CPU.to_string(), e.to_string()))?
        .split('\n')
        .filter(|l| l.starts_with(MODEL_NAME))
        .take(1)
        .collect::<String>()
        .split(':')
        .skip(1)
        .take(1)
        .collect::<String>()
        .trim()
        .to_string())
}

pub(crate) fn _cpu_cores() -> Result<u16, Error> {
    Ok(fs::read_to_string(CPU)
        .map_err(|e| Error::FileReadError(CPU.to_string(), e.to_string()))?
        .split('\n')
        .filter(|l| l.starts_with(CPU_CORES))
        .take(1)
        .collect::<String>()
        .split(':')
        .skip(1)
        .take(1)
        .collect::<String>()
        .trim()
        .parse::<u16>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn _logical_cores() -> Result<u16, Error> {
    Ok(fs::read_to_string(CPU)
        .map_err(|e| Error::FileReadError(CPU.to_string(), e.to_string()))?
        .split('\n')
        .filter(|l| l.starts_with(SIBLINGS))
        .take(1)
        .collect::<String>()
        .split(':')
        .skip(1)
        .take(1)
        .collect::<String>()
        .trim()
        .parse::<u16>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn _cpu_clock() -> Result<f32, Error> {
    Ok(fs::read_to_string(CPU)
        .map_err(|e| Error::FileReadError(CPU.to_string(), e.to_string()))?
        .split('\n')
        .filter(|l| l.starts_with(CPU_CLOCK))
        .take(1)
        .collect::<String>()
        .split(':')
        .skip(1)
        .take(1)
        .collect::<String>()
        .trim()
        .parse::<f32>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn _arch() -> Result<String, Error> {
    run(Command::new("uname").arg("-m"))
}

pub(crate) fn _memory() -> Result<usize, Error> {
    Ok(fs::read_to_string(MEM)
        .map_err(|e| Error::FileReadError(MEM.to_string(), e.to_string()))?
        .split('\n')
        .filter(|l| l.starts_with(TOTAL_MEM))
        .collect::<String>()
        .split_ascii_whitespace()
        .skip(1)
        .take(1)
        .collect::<String>()
        .parse::<usize>()
        .map_err(|e| Error::CommandParseError(e.to_string()))? as usize)
}

pub(crate) fn _swap() -> Result<usize, Error> {
    Ok(fs::read_to_string(MEM)
        .map_err(|e| Error::FileReadError(MEM.to_string(), e.to_string()))?
        .split('\n')
        .filter(|l| l.starts_with(TOTAL_SWAP))
        .collect::<String>()
        .split_ascii_whitespace()
        .skip(1)
        .take(1)
        .collect::<String>()
        .parse::<usize>()
        .map_err(|e| Error::CommandParseError(e.to_string()))? as usize)
}

pub(crate) fn _uptime() -> Result<u64, Error> {
    Ok(fs::read_to_string(UPTIME)
        .map_err(|e| Error::FileReadError(UPTIME.to_string(), e.to_string()))?
        .split_ascii_whitespace()
        .take(1)
        .collect::<String>()
        .parse::<f64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))? as u64)
}

/// Returns a kernel version of host os.
pub fn kernel_version() -> Result<String, Error> {
    Ok(fs::read_to_string(KERNEL).map_err(|e| Error::FileReadError(UPTIME.to_string(), e.to_string()))?)
}
