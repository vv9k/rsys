use crate::linux::net::{ipv4, ipv6, stat::IfaceStat};
use crate::linux::{SysFs, SysPath};
use crate::Result;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io};

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
