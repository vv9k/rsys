#[cfg(test)]
use super::NET_DEV;

use super::{ipv4, ipv6, Error, Result, SysFs, SysPath};
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
        let mut iface = Self::from_sys_path(SysFs::Sys.join("class").join("net").join(name), name)?;
        iface.stat = IfaceStat::from_proc(name)?;
        iface.ipv4 = ipv4(name)?;
        iface.ipv6 = ipv6(name)?;
        Ok(iface)
    }

    fn from_sys_path(path: SysPath, name: &str) -> Result<Interface> {
        Ok(Interface {
            name: name.to_string(),
            stat: IfaceStat::default(),
            mtu: path.clone().join("mtu").read_as::<u32>()?,
            mac_address: path.clone().join("address").read()?.trim().to_string(),
            speed: path.join("speed").read_as::<u64>().unwrap_or(0),
            ipv4: "".to_string(),
            ipv6: "".to_string(),
        })
    }

    /// Updates rx/tx stats
    pub fn update(&mut self) -> Result<()> {
        self.stat = IfaceStat::from_proc(&self.name)?;
        Ok(())
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
        Self::from_sys_path(SysFs::Proc.join("net").join("dev"), name)
    }

    fn from_sys_path(path: SysPath, name: &str) -> Result<IfaceStat> {
        for line in path.read()?.lines() {
            if line.contains(name) {
                return IfaceStat::from_line(line);
            }
        }

        Err(Error::InvalidInputError(
            SysFs::Proc.join("net").join("dev").path().to_string_lossy().to_string(),
            format!("interface {} not found in file", name),
        ))
    }

    fn from_line(line: &str) -> Result<IfaceStat> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io};
    #[test]
    fn parses_iface_stat() -> io::Result<()> {
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
        let dir = tempfile::tempdir()?;
        let p = dir.path().join("net_dev");
        fs::write(p.as_path(), NET_DEV)?;

        assert_eq!(
            Ok(lo),
            IfaceStat::from_sys_path(SysFs::Custom(p.clone()).as_syspath(), "lo")
        );
        assert_eq!(Ok(enp), IfaceStat::from_sys_path(SysFs::Custom(p).as_syspath(), "enp"));

        dir.close()
    }

    #[test]
    fn creates_interface() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::write(dir.path().join("speed"), b"1000")?;
        fs::write(dir.path().join("mtu"), b"1500")?;
        fs::write(dir.path().join("address"), b"70:85:c2:f9:9b:2a")?;

        let iface = Interface {
            name: "enp8s0".to_string(),
            speed: 1000,
            mtu: 1500,
            mac_address: "70:85:c2:f9:9b:2a".to_string(),
            ipv4: "".to_string(),
            ipv6: "".to_string(),
            stat: IfaceStat::default(),
        };

        assert_eq!(
            Ok(iface),
            Interface::from_sys_path(SysFs::Custom(dir.path().to_owned()).as_syspath(), "enp8s0")
        );

        dir.close()
    }
}
