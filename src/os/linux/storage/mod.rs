mod block;
mod cdrom;
mod info;
mod mappers;
mod mds;
mod partition;
mod stat;

pub use block::*;
pub use cdrom::*;
pub use info::*;
pub use mappers::*;
pub use mds::*;
pub use partition::*;

use crate::linux::{SysFs, SysPath};
use crate::{Error, Result};
use stat::BlockStorageStat;

use nix::sys::statfs;
use std::fs;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

//################################################################################
// Public
//################################################################################

/// All storage devices have a unique prefix like 'sd', 'dm', 'md', 'sr'... If a type implements
/// this trait it means it is one of the supported devices of linux kernel.
// #TODO: figure out if it's needed
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
    blk_bsz_get(SysFs::Dev.join(device).into_pathbuf().to_string_lossy().as_ref())
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

/// Returns a generic [`BlockStorageInfo`](BlockStorageInfo) for all encountered storage devices
/// the sysfs.
pub fn storage_devices_info() -> Result<Vec<BlockStorageInfo>> {
    let mut infos = Vec::new();
    for entry in SysFs::Sys.join("class").join("block").read_dir()? {
        if let Ok(entry) = entry {
            let dev_name = entry.file_name().to_string_lossy().to_string();
            infos.push(BlockStorageInfo::from_sys_path(
                &SysFs::Sys.join("class/block").join(&dev_name),
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
trait FromSysPath<T> {
    fn from_sys_path(path: &SysPath, hierarchy: bool, parse_stat: bool) -> Result<T>;
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Helper enum for generic function find_subdevices based on which
/// it decides to parse slaves or holders or nothing.
enum Hierarchy {
    Holders,
    Slaves,
    None,
}

fn stat<T: FromSysName<T>>(name: &str, parse_stat: bool) -> Result<T> {
    T::from_sys(name, parse_stat)
}

fn blk_bsz_get(path: &str) -> Result<i64> {
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
    device_path: &SysPath,
    holder_or_slave: Hierarchy,
    hierarchy: bool,
    parse_stat: bool,
) -> Option<Vec<T>> {
    let path = match holder_or_slave {
        Hierarchy::Holders => device_path.extend("holders"),
        Hierarchy::Slaves => device_path.extend("slaves"),
        Hierarchy::None => device_path.clone(),
    };

    let mut devs = Vec::new();
    if let Ok(dir) = fs::read_dir(path.as_path()) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if let Some(prefix) = prefix {
                        if !name.starts_with(prefix) {
                            continue;
                        }
                    }
                    if let Ok(dev) = T::from_sys_path(&path.extend(name), hierarchy, parse_stat) {
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
    device_path: &SysPath,
    holder_or_slave: Hierarchy,
    hierarchy: bool,
    parse_stat: bool,
) -> Option<Vec<T>> {
    _find_subdevices(Some(T::prefix()), device_path, holder_or_slave, hierarchy, parse_stat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_maj_min() {
        assert_eq!(parse_maj_min("X:5"), None);
        assert_eq!(parse_maj_min("1:Y"), None);
        assert_eq!(parse_maj_min("rand:"), None);
        assert_eq!(parse_maj_min(":xx"), None);
        assert_eq!(parse_maj_min("xxx"), None);
        assert_eq!(parse_maj_min("8:1"), Some((8, 1)))
    }
}
