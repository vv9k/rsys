#[cfg(test)]
use super::mocks::UPTIME;
use super::{run, SysPath};
use crate::{Error, Result};
use std::process::Command;

#[derive(Debug, Default, Eq, PartialEq)]
/// Represents a line of /proc/mounts
pub struct MountPoint {
    volume: String,
    path: String,
    voltype: String,
    options: String,
}
impl MountPoint {
    pub fn new(volume: &str, path: &str, voltype: &str, options: &str) -> MountPoint {
        MountPoint {
            volume: volume.to_string(),
            path: path.to_string(),
            voltype: voltype.to_string(),
            options: options.to_string(),
        }
    }
    pub(crate) fn from_line(line: &str) -> Option<MountPoint> {
        let mut elems = line.split_ascii_whitespace().take(4);
        if elems.clone().count() >= 4 {
            let volume = elems.next().unwrap();
            let path = elems.next().unwrap();
            let voltype = elems.next().unwrap();
            let options = elems.next().unwrap();
            return Some(Self::new(volume, path, voltype, options));
        }

        None
    }
}

pub type MountPoints = Vec<MountPoint>;

//--------------------------------------------------------------------------------
// API

/// Returns a hostname.
pub fn hostname() -> Result<String> {
    Ok(SysPath::ProcHostname.read()?.trim().to_string())
}

/// Internal implementation of parsing uptime from /proc/uptime
fn _uptime(out: &str) -> Result<u64> {
    Ok(out
        .split_ascii_whitespace()
        .take(1)
        .collect::<String>()
        .parse::<f64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))? as u64)
}

/// Returns current uptime.
pub fn uptime() -> Result<u64> {
    _uptime(&SysPath::ProcUptime.read()?)
}

/// Returns the processor architecture
pub fn arch() -> Result<String> {
    run(Command::new("uname").arg("-m"))
}

/// Returns a domainname read from /proc/sys/kernel/domainname
pub fn domainname() -> Result<String> {
    Ok(SysPath::ProcDomainName.read()?.trim().to_string())
}

/// Returns a kernel version of host os.
pub fn kernel_version() -> Result<String> {
    SysPath::ProcKernelRelease.read()
}

/// Returns MountPoints read from /proc/mounts
pub fn mounts() -> Result<MountPoints> {
    let mut mps = Vec::new();
    for line in SysPath::ProcMounts.read()?.split('\n') {
        if let Some(mp) = MountPoint::from_line(line) {
            mps.push(mp);
        }
    }
    Ok(mps)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gets_uptime() {
        assert_eq!(_uptime(UPTIME), Ok(5771))
    }
}
