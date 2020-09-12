use super::*;
use crate::util::trim_parse_map;
use std::fs;
use std::process::Command;

#[cfg(test)]
use super::mocks::{IP, IP_IFACE, ROUTE, UPTIME};

/// Returns a hostname.
pub fn hostname() -> Result<String, Error> {
    Ok(SysPath::ProcHostname.read()?.trim().to_string())
}

/// Internal implementation of parsing uptime from /proc/uptime
fn _uptime(out: &str) -> Result<u64, Error> {
    Ok(out
        .split_ascii_whitespace()
        .take(1)
        .collect::<String>()
        .parse::<f64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))? as u64)
}

/// Returns current uptime.
pub fn uptime() -> Result<u64, Error> {
    _uptime(&SysPath::ProcUptime.read()?)
}

/// Returns the processor architecture
pub fn arch() -> Result<String, Error> {
    run(Command::new("uname").arg("-m"))
}

/// Returns the name of first seen cpu in /proc/cpuinfo
pub fn cpu() -> Result<String, Error> {
    cpuinfo_extract::<String>(MODEL_NAME)
}

/// Returns cpu clock of first core in /proc/cpuinfo file.
pub fn cpu_clock() -> Result<f32, Error> {
    cpuinfo_extract::<f32>(CPU_CLOCK)
}

/// Returns total cpu cores available.
pub fn cpu_cores() -> Result<u16, Error> {
    cpuinfo_extract::<u16>(CPU_CORES)
}

/// Returns total logical cores available.
pub fn logical_cores() -> Result<u16, Error> {
    cpuinfo_extract::<u16>(SIBLINGS)
}

/// Returns total memory.
pub fn memory_total() -> Result<usize, Error> {
    mem_extract(MEM_TOTAL)
}

/// Returns free memory
pub fn memory_free() -> Result<usize, Error> {
    mem_extract(MEM_FREE)
}

/// Returns total swap space.
pub fn swap_total() -> Result<usize, Error> {
    mem_extract(SWAP_TOTAL)
}

/// Returns free swap space.
pub fn swap_free() -> Result<usize, Error> {
    mem_extract(SWAP_FREE)
}

/// Internal implementation of parsing default interface out from `route` command output
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

/// Returns a default interface.
pub fn default_iface() -> Result<String, Error> {
    let mut cmd = Command::new("route");
    _default_iface(&run(&mut cmd)?)
}

/// Internal implementation of parsing ipv4 value out from ip command output
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

/// Returns an IPv4 address of a given iface.
pub fn ipv4(iface: &str) -> Result<String, Error> {
    let out = ip(&iface)?;
    _ipv4(&out)
}

/// Returns an IPv6 address of a given iface.
pub fn ipv6(_iface: &str) -> Result<String, Error> {
    todo!()
}

/// Internal implementation of parsing mac value out from `ip addr show <iface>` command output
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

/// Returns a mac address of given iface
pub fn mac(iface: &str) -> Result<String, Error> {
    let out = ip(&iface)?;
    _mac(&out)
}

/// Internal implementation of parsing interfaces out from `ip address show` command output
fn _interfaces(out: &serde_json::Value) -> Result<Vec<String>, Error> {
    // It's ok to unwrap here because we check that out is an array and all non-string values are filtered out
    if !out.is_array() {
        return Err(Error::CommandParseError("invalid 'ip' command output".to_string()));
    }
    Ok(out
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["ifname"].is_string())
        .map(|v| v["ifname"].as_str().unwrap().to_string())
        .collect())
}

/// Returns a list of interfaces names.
pub fn interfaces() -> Result<Vec<String>, Error> {
    let out = ip("")?;
    _interfaces(&out)
}

/// Returns a domainname read from /proc/sys/kernel/domainname
pub fn domainname() -> Result<String, Error> {
    Ok(SysPath::ProcDomainName.read()?.trim().to_string())
}

//################################################################################
// UNIQUE

/// Returns a kernel version of host os.
pub fn kernel_version() -> Result<String, Error> {
    SysPath::ProcKernelRelease.read()
}

/// Returns MountPoints read from /proc/mounts
pub fn mounts() -> Result<MountPoints, Error> {
    let mut mps = Vec::new();
    for line in SysPath::ProcMounts.read()?.split('\n') {
        if let Some(mp) = MountPoint::from_line(line) {
            mps.push(mp);
        }
    }
    Ok(mps)
}

/// Returns Ifaces parsed from /proc/net/dev
pub fn ifaces() -> Result<Ifaces, Error> {
    let mut ifaces = Vec::new();
    for line in SysPath::ProcNetDev.read()?.split('\n') {
        if let Ok(iface) = IfaceDev::from_line(&line) {
            ifaces.push(iface)
        }
    }
    Ok(ifaces)
}

/// Returns detailed Process information parsed from /proc/[pid]/stat
pub fn stat_process(pid: i32) -> Result<Process, Error> {
    Process::from_stat(&SysPath::ProcPidStat(pid).read()?)
}

/// Returns a list of pids read from /proc
pub fn pids() -> Result<Vec<i32>, Error> {
    let path = SysPath::Proc.path();
    let mut pids = Vec::new();
    for _entry in fs::read_dir(&path).map_err(|e| Error::FileReadError(path, e.to_string()))? {
        if let Ok(entry) = _entry {
            let filename = entry.file_name();
            let sfilename = filename.as_os_str().to_string_lossy();
            if sfilename.chars().all(|c| c.is_digit(10)) {
                pids.push(
                    sfilename
                        .parse::<i32>()
                        .map_err(|e| Error::InvalidInputError(sfilename.to_string(), e.to_string()))?,
                );
            }
        }
    }
    Ok(pids)
}

/// Returns all processes currently seen in /proc parsed as Processes
pub fn processes() -> Result<Processes, Error> {
    let mut _pids = Vec::new();
    for pid in pids()? {
        _pids.push(stat_process(pid)?);
    }

    Ok(_pids)
}

pub fn stat_block_device(name: &str) -> Result<BlockStorage, Error> {
    fn maybe_value<T>(p: SysPath) -> Option<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        if let Ok(v) = p.read() {
            trim_parse_map::<T>(&v).ok()
        } else {
            None
        }
    }

    Ok(BlockStorage {
        dev: name.to_string(),
        size: trim_parse_map::<usize>(&SysPath::SysBlockDevSize(name).read()?)?,
        bd_model: maybe_value::<String>(SysPath::SysBlockDevModel(name)),
        bd_vendor: maybe_value::<String>(SysPath::SysBlockDevVendor(name)),
        bd_state: maybe_value::<String>(SysPath::SysBlockDevState(name)),
        stat: {
            if let Ok(stat) = SysPath::SysBlockDevStat(name).read() {
                BlockStorageStat::from_stat(&stat).ok()
            } else {
                None
            }
        },
        dm_name: maybe_value::<String>(SysPath::SysDevMapperName(name)),
        dm_uuid: maybe_value::<String>(SysPath::SysDevMapperUuid(name)),
    })
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
