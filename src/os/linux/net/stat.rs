use crate::linux::{SysFs, SysPath};
use crate::{Error, Result};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

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

impl IfaceStat {
    pub(crate) fn from_proc(name: &str) -> Result<IfaceStat> {
        Self::from_sys_path(&SysFs::Proc.join("net/dev"), name)
    }

    fn from_sys_path(path: &SysPath, name: &str) -> Result<IfaceStat> {
        for line in path.read()?.lines() {
            if line.contains(name) {
                return IfaceStat::from_line(line);
            }
        }

        Err(Error::InvalidInputError(
            SysFs::Proc.join("net/dev").to_pathbuf().to_string_lossy().to_string(),
            format!("interface {} not found in file", name),
        ))
    }

    fn from_line(line: &str) -> Result<IfaceStat> {
        let mut elems = line.split_ascii_whitespace().take(17);

        macro_rules! next_u64 {
            () => {
                elems
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .map_err(|e| Error::InvalidInputError(line.to_string(), e.to_string()))?
            };
        }

        if elems.clone().count() >= 17 {
            // skip interface name
            elems.next();
            return Ok(IfaceStat {
                rx_bytes: next_u64!(),
                rx_packets: next_u64!(),
                rx_errs: next_u64!(),
                rx_drop: next_u64!(),
                rx_fifo: next_u64!(),
                rx_frame: next_u64!(),
                rx_compressed: next_u64!(),
                rx_multicast: next_u64!(),

                tx_bytes: next_u64!(),
                tx_packets: next_u64!(),
                tx_errs: next_u64!(),
                tx_drop: next_u64!(),
                tx_fifo: next_u64!(),
                tx_frame: next_u64!(),
                tx_compressed: next_u64!(),
                tx_multicast: next_u64!(),
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
    use crate::linux::mocks::NET_DEV;
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

        let path = SysFs::Custom(p).to_syspath();
        assert_eq!(Ok(lo), IfaceStat::from_sys_path(&path, "lo"));
        assert_eq!(Ok(enp), IfaceStat::from_sys_path(&path, "enp"));

        dir.close()
    }
}
