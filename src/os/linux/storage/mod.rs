mod block;
mod cdrom;
mod mappers;
mod mds;
mod partition;
mod stat;

pub use block::*;
pub use cdrom::*;
pub use mappers::*;
pub use mds::*;
pub use partition::*;

use crate::linux::SysFs;
use crate::{util::trim_parse_map, Error, Result};
use stat::BlockStorageStat;

use nix::sys::statfs;
use std::fs;
use std::path::PathBuf;

//################################################################################
// Public
//################################################################################

#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Common information of each type of storage device.
pub struct BlockStorageInfo {
    /// Device name like "sda"
    pub dev: String,
    /// Size in bytes
    pub size: usize,
    /// Major number identifies the driver associated with this device
    pub maj: u32,
    /// Minor number
    pub min: u32,
    /// Size of one block
    pub block_size: i64,
    /// I/O stats of this device
    pub stat: Option<BlockStorageStat>,
    pub(crate) path: PathBuf,
}
impl BlockStorageInfo {
    pub(crate) fn from_sys_path(path: PathBuf, parse_stat: bool) -> Result<BlockStorageInfo> {
        let (maj, min) = parse_maj_min(&SysFs::Custom(path.clone()).join("dev").read()?).unwrap_or_default();
        let device = path
            .clone()
            .file_name()
            .ok_or_else(|| {
                Error::InvalidInputError(
                    path.to_string_lossy().to_string(),
                    "Given path doesn't have a file name".to_string(),
                )
            })?
            .to_string_lossy()
            .to_string();
        Ok(BlockStorageInfo {
            dev: device.clone(),
            size: trim_parse_map::<usize>(&SysFs::Custom(path.clone()).join("size").read()?)?,
            maj,
            min,
            block_size: block_size(&device)?,
            stat: if parse_stat {
                Some(BlockStorageStat::from_stat(
                    &SysFs::Custom(path.clone()).join("stat").read()?,
                )?)
            } else {
                None
            },
            path,
        })
    }
    pub fn update_stats(&mut self) -> Result<()> {
        self.stat = Some(BlockStorageStat::from_stat(
            &SysFs::Custom(self.path.clone()).join("stat").read()?,
        )?);
        Ok(())
    }
}

/// All storage devices have a unique prefix like 'sd', 'dm', 'md', 'sr'... If a type implements
/// this trait it means it is one of the supported devices of linux kernel.
pub trait BlockStorageDeviceName {
    fn prefix() -> &'static str;
}

// #TODO: document this trait, figure out if it's needed
pub trait FromSysName<T> {
    fn from_sys(name: &str, parse_stat: bool) -> Result<T>;
}

/// Returns block size of device in bytes
/// device argument must be a path to block device file descriptor
pub fn block_size(device: &str) -> Result<i64> {
    blk_bsz_get(SysFs::Dev.join(device).path().to_string_lossy().as_ref())
}

/// Parses a StorageDevice object from system. If the provided name
/// doesn't start with `sd` returns an error.
///
/// parse_stats decides whether or not to parse statistics for slave/holder devices,
/// when this parameter is on and there are a lot of devices on host os the output
/// might get quite large.
pub fn stat_block_device(name: &str, parse_stats: bool) -> Result<StorageDevice> {
    stat::<StorageDevice>(name, parse_stats)
}

/// Parses a DeviceMapper object from system. If the provided name
/// doesn't start with `dm` returns an error.
pub fn stat_device_mapper(name: &str, parse_stats: bool) -> Result<DeviceMapper> {
    stat::<DeviceMapper>(name, parse_stats)
}

/// Parses a ScsiCdrom object from system. If the provided name
/// doesn't start with `sr` returns an error.
pub fn stat_scsi_cdrom(name: &str, parse_stats: bool) -> Result<ScsiCdrom> {
    stat::<ScsiCdrom>(name, parse_stats)
}

/// Parses a MultipleDeviceStorage object from system. If the provided name
/// doesn't start with `md` returns an error.
pub fn stat_multiple_device_storage(name: &str, parse_stats: bool) -> Result<MultipleDeviceStorage> {
    stat::<MultipleDeviceStorage>(name, parse_stats)
}

/// Parses multiple storage devices of type T from filesystem
pub fn storage_devices<T: FromSysName<T> + BlockStorageDeviceName>(parse_stats: bool) -> Result<Vec<T>> {
    let mut devices = Vec::new();
    for entry in SysFs::Sys.join("block").read_dir()? {
        if let Ok(entry) = entry {
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(T::prefix()) {
                devices.push(stat::<T>(
                    &entry.file_name().to_string_lossy().to_string(),
                    parse_stats,
                )?);
            }
        }
    }

    Ok(devices)
}

pub fn storage_devices_info() -> Result<Vec<BlockStorageInfo>> {
    let mut infos = Vec::new();
    for entry in SysFs::Sys.join("class").join("block").read_dir()? {
        if let Ok(entry) = entry {
            let dev_name = entry.file_name().to_string_lossy().to_string();
            infos.push(BlockStorageInfo::from_sys_path(
                SysFs::Sys.join("class").join("block").join(&dev_name).path(),
                true,
            )?);
        }
    }
    Ok(infos)
}

//################################################################################
// Internal
//################################################################################

/// Helper trait for generic parsing of storage devices from /sys/block/<dev>.
/// parse_stat decides whether or not to parse stats for slave/holder devices.
pub(crate) trait FromSysPath<T> {
    fn from_sys_path(path: PathBuf, hierarchy: bool, parse_stat: bool) -> Result<T>;
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Helper enum for generic function find_subdevices based on which
/// it decides to parse slaves or holders or nothing.
pub(crate) enum Hierarchy {
    Holders,
    Slaves,
    None,
}

pub(crate) fn stat<T: FromSysName<T>>(name: &str, parse_stat: bool) -> Result<T> {
    T::from_sys(name, parse_stat)
}

pub(crate) fn blk_bsz_get(path: &str) -> Result<i64> {
    statfs::statfs(path).map(|v| v.block_size() as i64).map_err(Error::from)
}

// Parses out major and minor number from str like '8:1'
// and returns a tuple (8, 1)
fn parse_maj_min(dev: &str) -> Option<(u32, u32)> {
    let mut elems = dev.split(':');

    if let Some(maj) = elems.next() {
        if let Some(min) = elems.next() {
            if let Ok(maj) = maj.trim().parse::<u32>() {
                if let Ok(min) = min.trim().parse::<u32>() {
                    return Some((maj, min));
                }
            }
        }
    }

    None
}

fn _find_subdevices<T: FromSysPath<T>>(
    prefix: Option<&str>,
    mut device_path: PathBuf,
    holder_or_slave: Hierarchy,
    hierarchy: bool,
    parse_stat: bool,
) -> Option<Vec<T>> {
    match holder_or_slave {
        Hierarchy::Holders => device_path.push("holders"),
        Hierarchy::Slaves => device_path.push("slaves"),
        Hierarchy::None => {}
    };

    let mut devs = Vec::new();
    if let Ok(dir) = fs::read_dir(device_path.as_path()) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if let Some(prefix) = prefix {
                        if !name.starts_with(prefix) {
                            continue;
                        }
                    }
                    if let Ok(dev) = T::from_sys_path(device_path.join(name), hierarchy, parse_stat) {
                        devs.push(dev);
                    }
                }
            }
        }
        if !devs.is_empty() {
            return Some(devs);
        }
    }

    None
}

fn find_subdevices<T: FromSysPath<T> + BlockStorageDeviceName>(
    device_path: PathBuf,
    holder_or_slave: Hierarchy,
    hierarchy: bool,
    parse_stat: bool,
) -> Option<Vec<T>> {
    _find_subdevices(Some(T::prefix()), device_path, holder_or_slave, hierarchy, parse_stat)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linux::mocks::SYS_BLOCK_DEV_STAT;
    use std::{fs, io};

    #[test]
    fn parses_maj_min() {
        assert_eq!(parse_maj_min("X:5"), None);
        assert_eq!(parse_maj_min("1:Y"), None);
        assert_eq!(parse_maj_min("rand:"), None);
        assert_eq!(parse_maj_min(":xx"), None);
        assert_eq!(parse_maj_min("xxx"), None);
        assert_eq!(parse_maj_min("8:1"), Some((8, 1)))
    }

    #[test]
    fn parses_block_storage_info() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let p = dir.path().join("sda");

        fs::create_dir(p.as_path())?;

        fs::write(p.as_path().join("stat"), SYS_BLOCK_DEV_STAT)?;
        fs::write(p.as_path().join("dev"), b"8:0")?;
        fs::write(p.as_path().join("size"), b"3907029168")?;

        let mut info = BlockStorageInfo {
            dev: "sda".to_string(),
            // This probably shouldn't be here
            block_size: 4096,
            path: p.clone(),
            size: 3907029168,
            maj: 8,
            min: 0,
            stat: Some(BlockStorageStat {
                read_ios: 327,
                read_merges: 72,
                read_sectors: 8832,
                read_ticks: 957,
                write_ios: 31,
                write_merges: 1,
                write_sectors: 206,
                write_ticks: 775,
                in_flight: 0,
                io_ticks: 1620,
                time_in_queue: 2427,
                discard_ios: 0,
                discard_merges: 0,
                discard_sectors: 0,
                discard_ticks: 0,
            }),
        };

        assert_eq!(Ok(info.clone()), BlockStorageInfo::from_sys_path(p.clone(), true));

        info.stat = None;

        assert_eq!(Ok(info), BlockStorageInfo::from_sys_path(p, false));

        dir.close()
    }
}
