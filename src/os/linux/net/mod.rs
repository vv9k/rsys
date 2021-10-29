pub(crate) mod iface;
pub(crate) mod stat;

pub use iface::*;
pub use stat::*;

use crate::linux::SysFs;
use crate::Result;

use nix::{
    ifaddrs::getifaddrs,
    sys::socket::{InetAddr, SockAddr},
};

//################################################################################
// Public
//################################################################################

/// Returns a default interface. If there are no interfaces in /proc/net/arp
/// returns an empty string.
pub fn default_iface() -> Result<String> {
    if let Some(line) = SysFs::Proc.join("net/arp").read()?.lines().nth(1) {
        if let Some(name) = line.split_ascii_whitespace().last() {
            return Ok(name.to_string());
        }
    }
    Ok("".to_string())
}

/// Returns an IPv4 address of a given iface. If the interface is not
/// found in /proc/net/arp returns "127.0.0.1"
pub fn ipv4(iface: &str) -> Result<String> {
    if let Some(ip) = _ip(iface, false)? {
        Ok(ip)
    } else {
        Ok("".to_string())
    }
}

/// Returns an IPv6 address of a given iface.
pub fn ipv6(iface: &str) -> Result<String> {
    if let Some(ip) = _ip(iface, true)? {
        Ok(ip)
    } else {
        Ok("".to_string())
    }
}

/// Returns a mac address of given iface
pub fn mac(iface: &str) -> Result<String> {
    Ok(SysFs::Sys.join("class/net").join(iface).read()?.trim().to_string())
}

/// Returns a list of interfaces names.
pub fn interfaces() -> Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in SysFs::Sys.join("class/net").read_dir()?.flatten() {
        names.push(entry.file_name().to_string_lossy().to_string());
    }
    Ok(names)
}

/// Returns network interfaces on host os
pub fn ifaces() -> Result<Interfaces> {
    let mut ifaces = Vec::new();
    for entry in SysFs::Sys.join("class/net").read_dir()?.flatten() {
        if let Some(filename) = entry.file_name().to_str() {
            ifaces.push(Interface::from_sys(filename)?);
        }
    }
    Ok(Interfaces(ifaces))
}

pub fn iface(name: &str) -> Result<Option<Interface>> {
    for entry in SysFs::Sys.join("class/net").read_dir()?.flatten() {
        if let Some(filename) = entry.file_name().to_str() {
            if filename == name {
                return Ok(Some(Interface::from_sys(filename)?));
            }
        }
    }
    Ok(None)
}

//################################################################################
// Internal
//################################################################################

fn _ip(name: &str, v6: bool) -> Result<Option<String>> {
    for iface in getifaddrs()?.into_iter().filter(|iface| iface.interface_name == name) {
        if let Some(addr) = iface.address {
            match addr {
                SockAddr::Inet(ip) => match ip {
                    InetAddr::V4(_) if !v6 => {
                        let addr = addr.to_str();
                        // skip :<port>
                        if let Some(last_idx) = addr.rfind(':') {
                            return Ok(Some(addr[..last_idx].to_string()));
                        }
                        return Ok(Some(addr));
                    }
                    InetAddr::V6(_) if v6 => {
                        let addr = addr.to_str();
                        // skip [ ]:<port>
                        if let Some(last_idx) = addr.rfind(']') {
                            return Ok(Some(addr[1..last_idx].to_string()));
                        }
                        return Ok(Some(addr));
                    }
                    _ => continue,
                },
                _ => continue,
            }
        }
    }
    Ok(None)
}
