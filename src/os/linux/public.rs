use super::*;
use std::process::Command;

pub fn hostname() -> Result<String, Error> {
    Ok(ProcPath::Hostname.read()?.trim().to_string())
}

pub fn uptime() -> Result<u64, Error> {
    Ok(ProcPath::Uptime
        .read()?
        .split_ascii_whitespace()
        .take(1)
        .collect::<String>()
        .parse::<f64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))? as u64)
}

pub fn arch() -> Result<String, Error> {
    run(Command::new("uname").arg("-m"))
}

pub fn cpu() -> Result<String, Error> {
    cpuinfo_extract::<String>(MODEL_NAME)
}

pub fn cpu_clock() -> Result<f32, Error> {
    cpuinfo_extract::<f32>(CPU_CLOCK)
}

pub fn cpu_cores() -> Result<u16, Error> {
    cpuinfo_extract::<u16>(CPU_CORES)
}

pub fn logical_cores() -> Result<u16, Error> {
    cpuinfo_extract::<u16>(SIBLINGS)
}

pub fn memory_total() -> Result<usize, Error> {
    mem_extract(MEM_TOTAL)
}

pub fn memory_free() -> Result<usize, Error> {
    mem_extract(MEM_FREE)
}

pub fn swap_total() -> Result<usize, Error> {
    mem_extract(SWAP_TOTAL)
}

pub fn swap_free() -> Result<usize, Error> {
    mem_extract(SWAP_FREE)
}

pub fn default_iface() -> Result<String, Error> {
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

pub fn ipv4(iface: &str) -> Result<String, Error> {
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

pub fn ipv6(_iface: &str) -> Result<String, Error> {
    todo!()
}

pub fn mac(iface: &str) -> Result<String, Error> {
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

pub fn interfaces() -> Result<Vec<String>, Error> {
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

pub fn domainname() -> Result<String, Error> {
    Ok(ProcPath::DomainName.read()?.trim().to_string())
}

//################################################################################
// UNIQUE

/// Returns a kernel version of host os.
pub fn kernel_version() -> Result<String, Error> {
    ProcPath::KernelRelease.read()
}
