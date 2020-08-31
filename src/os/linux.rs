use super::{run, Error};
use std::fs;
use std::process::Command;

const HOSTNAME: &str = "/etc/hostname";
const CPU: &str = "/proc/cpuinfo";
const MEM: &str = "/proc/meminfo";
const UPTIME: &str = "/proc/uptime";

const MODEL_NAME: &str = "model name";
const TOTAL_MEM: &str = "MemTotal:       ";

pub fn default_iface() -> Result<String, Error> {
    let mut cmd = Command::new("roaute");
    Ok(run(&mut cmd)?
        .split('\n')
        .filter(|l| l.starts_with("default"))
        .collect::<String>()
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .to_string())
}

pub fn hostname() -> Result<String, Error> {
    Ok(fs::read_to_string(HOSTNAME)
        .map_err(|e| Error::FileReadError(HOSTNAME.to_string(), e.to_string()))?
        .trim()
        .to_string())
}

fn ip(ifc: &str) -> Result<serde_json::Value, Error> {
    Ok(
        serde_json::from_str::<serde_json::Value>(&run(Command::new("ip")
            .arg("-j")
            .arg("-p")
            .arg("address")
            .arg("show")
            .arg(ifc))?)
        .map_err(|e| Error::CommandParseError(e.to_string()))?,
    )
}

pub fn ipv4(iface: &str) -> Result<String, Error> {
    let out = ip(&iface)?;
    let ip = &out[0]["addr_info"][0]["local"];
    if ip.is_string() {
        // It's ok to unwrap here because we know it's a string
        return Ok(ip.as_str().map(|s| s.to_string()).unwrap());
    }

    Ok("127.0.0.1".to_string())
}

pub fn ipv6() -> Result<String, Error> {
    todo!()
}

pub fn cpu() -> Result<String, Error> {
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

pub fn arch() -> Result<String, Error> {
    run(Command::new("uname").arg("-m"))
}

pub fn memory() -> Result<usize, Error> {
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

pub fn uptime() -> Result<u64, Error> {
    Ok(fs::read_to_string(UPTIME)
        .map_err(|e| Error::FileReadError(UPTIME.to_string(), e.to_string()))?
        .split_ascii_whitespace()
        .take(1)
        .collect::<String>()
        .parse::<f64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))? as u64)
}
