use super::{ipv4, ipv6, Error, Result, SysPath};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// A wrapper around multiple interface devices parsed from /proc/net/dev
pub struct Interfaces(pub Vec<Interface>);

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a network interface on host os
pub struct Interface {
    /// Name of this interface
    pub name: String,
    /// IPv4 address of this interface
    pub ipv4: String,
    /// IPv6 address of this interface
    pub ipv6: String,
    /// Stats of this interface from /proc/net/dev
    pub stat: IfaceStat,
    /// Maximum transmission unit
    pub mtu: u32,
    pub mac_address: String,
    /// Speed in mb/s
    pub speed: u64,
}
impl Interface {
    pub(crate) fn from_sys(name: &str) -> Result<Interface> {
        Ok(Interface {
            name: name.to_string(),
            stat: IfaceStat::from_proc(name)?,
            mtu: SysPath::SysClassNetDev(name).extend(&["mtu"]).read_as::<u32>()?,
            mac_address: SysPath::SysClassNetDev(name)
                .extend(&["address"])
                .read()?
                .trim()
                .to_string(),
            speed: SysPath::SysClassNetDev(name)
                .extend(&["speed"])
                .read_as::<u64>()
                .unwrap_or_else(|_| 0),
            ipv4: ipv4(name)?,
            ipv6: ipv6(name)?,
        })
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Statistics of network interface device read from /proc/net/dev
pub struct IfaceStat {
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

macro_rules! next_u64 {
    ($list:ident, $line:ident) => {
        $list
            .next()
            .unwrap()
            .parse::<u64>()
            .map_err(|e| Error::InvalidInputError($line.to_string(), e.to_string()))?
    };
}

impl IfaceStat {
    pub(crate) fn from_proc(name: &str) -> Result<IfaceStat> {
        for line in SysPath::ProcNetDev.read()?.lines() {
            if line.contains(name) {
                return IfaceStat::from_line(line);
            }
        }

        Err(Error::InvalidInputError(
            SysPath::ProcNetDev.path().to_string_lossy().to_string(),
            format!("interface {} not found in file", name),
        ))
    }
    pub(crate) fn from_line(line: &str) -> Result<IfaceStat> {
        let mut elems = line.split_ascii_whitespace().take(17);
        if elems.clone().count() >= 17 {
            // skip interface name
            elems.next();
            return Ok(IfaceStat {
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
