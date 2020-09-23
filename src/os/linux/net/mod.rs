#[cfg(test)]
use super::mocks::{IP, IP_IFACE, NET_DEV, ROUTE};
use super::{run, SysPath};
use crate::{Error, Result};
use std::process::Command;

pub type Ifaces = Vec<IfaceDev>;

macro_rules! next_u64 {
    ($list:ident, $line:ident) => {
        $list
            .next()
            .unwrap()
            .parse::<u64>()
            .map_err(|e| Error::InvalidInputError($line.to_string(), e.to_string()))?
    };
}

#[derive(Debug, Default, Eq, PartialEq)]
/// Represents a data line of /proc/net/dev
pub struct IfaceDev {
    pub iface: String,

    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errs: u64,
    pub rx_drop: u64,
    pub rx_fifo: u64,
    pub rx_frame: u64,
    pub rx_compressed: u64,
    pub rx_multicast: u64,

    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errs: u64,
    pub tx_drop: u64,
    pub tx_fifo: u64,
    pub tx_frame: u64,
    pub tx_compressed: u64,
    pub tx_multicast: u64,
}
impl IfaceDev {
    pub(crate) fn from_line(line: &str) -> Result<IfaceDev> {
        let mut elems = line.split_ascii_whitespace().take(17);
        if elems.clone().count() >= 17 {
            return Ok(IfaceDev {
                iface: elems.next().unwrap().trim_end_matches(':').to_string(),

                rx_bytes: next_u64!(elems, line),
                rx_packets: next_u64!(elems, line),
                rx_errs: next_u64!(elems, line),
                rx_drop: next_u64!(elems, line),
                rx_fifo: next_u64!(elems, line),
                rx_frame: next_u64!(elems, line),
                rx_compressed: next_u64!(elems, line),
                rx_multicast: next_u64!(elems, line),

                tx_bytes: next_u64!(elems, line),
                tx_packets: next_u64!(elems, line),
                tx_errs: next_u64!(elems, line),
                tx_drop: next_u64!(elems, line),
                tx_fifo: next_u64!(elems, line),
                tx_frame: next_u64!(elems, line),
                tx_compressed: next_u64!(elems, line),
                tx_multicast: next_u64!(elems, line),
            });
        }

        Err(Error::InvalidInputError(
            line.to_string(),
            "contains invalid input for IfaceDev".to_string(),
        ))
    }
}

pub(crate) fn ip(iface: &str) -> Result<serde_json::Value> {
    let mut _ip = Command::new("ip");
    let mut cmd = if iface == "" {
        _ip.arg("-j").arg("address").arg("show")
    } else {
        _ip.arg("-j").arg("address").arg("show").arg(&iface)
    };
    Ok(serde_json::from_str::<serde_json::Value>(&run(&mut cmd)?)
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

//--------------------------------------------------------------------------------
// API

/// Internal implementation of parsing default interface out from `route` command output
fn _default_iface(out: &str) -> Result<String> {
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
pub fn default_iface() -> Result<String> {
    let mut cmd = Command::new("route");
    _default_iface(&run(&mut cmd)?)
}

/// Internal implementation of parsing ipv4 value out from ip command output
fn _ipv4(out: &serde_json::Value) -> Result<String> {
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
pub fn ipv4(iface: &str) -> Result<String> {
    let out = ip(&iface)?;
    _ipv4(&out)
}

/// Returns an IPv6 address of a given iface.
pub fn ipv6(_iface: &str) -> Result<String> {
    todo!()
}

/// Internal implementation of parsing mac value out from `ip addr show <iface>` command output
fn _mac(out: &serde_json::Value) -> Result<String> {
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
pub fn mac(iface: &str) -> Result<String> {
    let out = ip(&iface)?;
    _mac(&out)
}

/// Internal implementation of parsing interfaces out from `ip address show` command output
fn _interfaces(out: &serde_json::Value) -> Result<Vec<String>> {
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
pub fn interfaces() -> Result<Vec<String>> {
    let out = ip("")?;
    _interfaces(&out)
}

/// Returns Ifaces parsed from /proc/net/dev
pub fn ifaces() -> Result<Ifaces> {
    let mut ifaces = Vec::new();
    for line in SysPath::ProcNetDev.read()?.split('\n') {
        if let Ok(iface) = IfaceDev::from_line(&line) {
            ifaces.push(iface)
        }
    }
    Ok(ifaces)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_iface_from_net_dev_line() {
        let lo = IfaceDev {
            iface: "lo".to_string(),
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
        let enp = IfaceDev {
            iface: "enp8s0".to_string(),
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

        assert_eq!(Ok(lo), IfaceDev::from_line(lines.next().unwrap()));
        assert_eq!(Ok(enp), IfaceDev::from_line(lines.next().unwrap()))
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
