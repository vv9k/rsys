#[cfg(test)]
use super::mocks::*;
use super::*;
use std::process::Command;

pub fn hostname() -> Result<String, Error> {
    Ok(ProcPath::Hostname.read()?.trim().to_string())
}

fn _uptime(out: &str) -> Result<u64, Error> {
    Ok(out
        .split_ascii_whitespace()
        .take(1)
        .collect::<String>()
        .parse::<f64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))? as u64)
}

pub fn uptime() -> Result<u64, Error> {
    _uptime(&ProcPath::Uptime.read()?)
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

fn _default_iface(out: &str) -> Result<String, Error> {
    Ok(out
        .split('\n')
        .filter(|l| l.starts_with("default"))
        .collect::<String>()
        .split_ascii_whitespace()
        .last()
        .ok_or_else(|| Error::CommandParseError("output of route command was invalid".to_string()))?
        .to_string())
}

pub fn default_iface() -> Result<String, Error> {
    let mut cmd = Command::new("route");
    _default_iface(&run(&mut cmd)?)
}

fn _ipv4(out: &serde_json::Value) -> Result<String, Error> {
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

pub fn ipv4(iface: &str) -> Result<String, Error> {
    let out = ip(&iface)?;
    _ipv4(&out)
}

pub fn ipv6(_iface: &str) -> Result<String, Error> {
    todo!()
}

fn _mac(out: &serde_json::Value) -> Result<String, Error> {
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

pub fn mac(iface: &str) -> Result<String, Error> {
    let out = ip(&iface)?;
    _mac(&out)
}

fn _interfaces(out: &serde_json::Value) -> Result<Vec<String>, Error> {
    // It's ok to unwrap here because we check that out is an array and all non-string values are filtered out
    Ok(out
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["ifname"].is_string())
        .map(|v| v["ifname"].as_str().unwrap().to_string())
        .collect())
}

pub fn interfaces() -> Result<Vec<String>, Error> {
    let out = ip("")?;
    if !out.is_array() {
        return Err(Error::CommandParseError("invalid 'ip' command output".to_string()));
    }
    _interfaces(&out)
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

pub fn mounts() -> Result<MountPoints, Error> {
    let mut mps = Vec::new();
    for line in ProcPath::Mounts.read()?.split('\n') {
        if let Some(mp) = MountPoint::from_line(line) {
            mps.push(mp);
        }
    }
    Ok(mps)
}

fn _ifaces(out: &str) -> Result<Ifaces, Error> {
    let mut ifaces = Vec::new();
    for line in out.split('\n') {
        if let Ok(iface) = IfaceDev::from_line(&line) {
            ifaces.push(iface)
        }
    }
    Ok(ifaces)
}

pub fn ifaces() -> Result<Ifaces, Error> {
    _ifaces(&ProcPath::NetDev.read()?)
}

fn _stat_process(out: &str) -> Result<Process, Error> {
    Process::from_stat(out)
}

pub fn stat_process(pid: u64) -> Result<Process, Error> {
    _stat_process(&ProcPath::PidStat(pid).read()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gets_uptime() {
        assert_eq!(_uptime(UPTIME), Ok(5771))
    }

    #[test]
    fn gets_default_iface() {
        assert_eq!(_default_iface(ROUTE), Ok("enp8s0".to_string()))
    }

    #[test]
    fn gets_ipv4() {
        assert_eq!(
            _ipv4(&serde_json::from_str::<serde_json::Value>(IP_IFACE).unwrap()),
            Ok("192.168.0.6".to_string())
        )
    }

    #[test]
    fn gets_mac() {
        assert_eq!(
            _mac(&serde_json::from_str::<serde_json::Value>(IP_IFACE).unwrap()),
            Ok("70:85:c2:f9:9b:2a".to_string())
        )
    }

    #[test]
    fn gets_interfaces() {
        assert_eq!(
            _interfaces(&serde_json::from_str::<serde_json::Value>(IP).unwrap()),
            Ok(vec!["lo", "enp8s0", "br-211476fe73de", "docker0"]
                .into_iter()
                .map(str::to_string)
                .collect())
        )
    }
}
