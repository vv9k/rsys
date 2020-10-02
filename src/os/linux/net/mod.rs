mod types;
#[cfg(test)]
use super::mocks::NET_DEV;
use super::SysPath;
use crate::{Error, Result};
pub use types::*;

fn _ipv4(name: &str) -> Result<Option<String>> {
    for line in SysPath::ProcNetArp.read()?.lines() {
        if line.ends_with(name) {
            return Ok(line.split_ascii_whitespace().next().map(|s| s.to_string()));
        }
    }
    Ok(None)
}

/// Returns a default interface. If there are no interfaces in /proc/net/arp
/// returns an empty string.
pub fn default_iface() -> Result<String> {
    if let Some(line) = SysPath::ProcNetArp.read()?.lines().skip(1).next() {
        if let Some(name) = line.split_ascii_whitespace().last() {
            return Ok(name.to_string());
        }
    }
    Ok("".to_string())
}

/// Returns an IPv4 address of a given iface. If the interface is not
/// found in /proc/net/arp returns "127.0.0.1"
pub fn ipv4(iface: &str) -> Result<String> {
    if let Some(ip) = _ipv4(&iface)? {
        Ok(ip)
    } else {
        Ok("127.0.0.1".to_string())
    }
}

/// Returns an IPv6 address of a given iface.
pub fn ipv6(_iface: &str) -> Result<String> {
    todo!()
}

/// Returns a mac address of given iface
pub fn mac(iface: &str) -> Result<String> {
    Ok(SysPath::SysClassNetDev(iface).read()?.trim().to_string())
}

/// Returns a list of interfaces names.
pub fn interfaces() -> Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in SysPath::SysClassNet.read_dir()? {
        if let Ok(entry) = entry {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    Ok(names)
}

/// Returns network interfaces on host os
pub fn ifaces() -> Result<Interfaces> {
    let mut ifaces = Vec::new();
    for entry in SysPath::SysClassNet.read_dir()? {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                ifaces.push(Interface::from_sys(filename)?);
            }
        }
    }
    Ok(Interfaces(ifaces))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_iface_from_net_dev_line() {
        let lo = IfaceStat {
            rx_bytes: 17776656,
            rx_packets: 127989,
            rx_errs: 0,
            rx_drop: 0,
            rx_fifo: 0,
            rx_frame: 0,
            rx_compressed: 0,
            rx_multicast: 0,

            tx_bytes: 17776656,
            tx_packets: 127989,
            tx_errs: 0,
            tx_drop: 0,
            tx_fifo: 0,
            tx_frame: 0,
            tx_compressed: 0,
            tx_multicast: 0,
        };
        let enp = IfaceStat {
            rx_bytes: 482459368,
            rx_packets: 349468,
            rx_errs: 0,
            rx_drop: 0,
            rx_fifo: 0,
            rx_frame: 0,
            rx_compressed: 0,
            rx_multicast: 4785,

            tx_bytes: 16133415,
            tx_packets: 198549,
            tx_errs: 0,
            tx_drop: 0,
            tx_fifo: 0,
            tx_frame: 0,
            tx_compressed: 0,
            tx_multicast: 0,
        };
        let mut lines = NET_DEV.split('\n').skip(2);

        assert_eq!(Ok(lo), IfaceStat::from_line(lines.next().unwrap()));
        assert_eq!(Ok(enp), IfaceStat::from_line(lines.next().unwrap()))
    }
}
