mod system;
mod types;

pub use types::*;

#[cfg(test)]
pub(crate) use super::mocks::SYS_BLOCK_DEV_STAT;
use super::{storage::stat, SysPath};
use crate::Result;
use system::blk_bsz_get;

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

/// Returns block size of device in bytes
/// device argument must be a path to block device file descriptor
pub fn block_size(device: &str) -> Result<i64> {
    blk_bsz_get(SysPath::Dev(device).path().to_string_lossy().as_ref())
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
    for entry in SysPath::SysBlock.read_dir()? {
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
    for entry in SysPath::SysClassBlock.read_dir()? {
        if let Ok(entry) = entry {
            let dev_name = entry.file_name().to_string_lossy().to_string();
            infos.push(BlockStorageInfo::from_sys_path(
                SysPath::SysClassBlock.extend(&[&dev_name]).path(),
                true,
            )?);
        }
    }
    Ok(infos)
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
