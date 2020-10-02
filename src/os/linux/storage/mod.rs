mod system;
mod types;

pub use types::*;

#[cfg(test)]
use super::mocks::SYS_BLOCK_DEV_STAT;
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
pub fn stat_block_device(name: &str) -> Result<StorageDevice> {
    stat::<StorageDevice>(name)
}

/// Parses a DeviceMapper object from system. If the provided name
/// doesn't start with `dm` returns an error.
pub fn stat_device_mapper(name: &str) -> Result<DeviceMapper> {
    stat::<DeviceMapper>(name)
}

/// Parses a ScsiCdrom object from system. If the provided name
/// doesn't start with `sr` returns an error.
pub fn stat_scsi_cdrom(name: &str) -> Result<ScsiCdrom> {
    stat::<ScsiCdrom>(name)
}

/// Parses a MultipleDeviceStorage object from system. If the provided name
/// doesn't start with `md` returns an error.
pub fn stat_multiple_device_storage(name: &str) -> Result<MultipleDeviceStorage> {
    stat::<MultipleDeviceStorage>(name)
}

/// Parses multiple storage devices of type T from filesystem
pub fn storage_devices<T: FromSysName<T> + BlockStorageDeviceName>() -> Result<Vec<T>> {
    let mut devices = Vec::new();
    for entry in SysPath::SysBlock.read_dir()? {
        if let Ok(entry) = entry {
            let filename = entry.file_name().to_string_lossy().to_string();
            if filename.starts_with(T::prefix()) {
                devices.push(stat::<T>(&entry.file_name().to_string_lossy().to_string())?);
            }
        }
    }

    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_block_device_stat_from_sys_block_dev_stat() {
        let dev = StorageDevice {
            model: "ST2000DM008-2FR1".to_string(),
            vendor: "ATA".to_string(),
            state: "running".to_string(),
            info: BlockStorageInfo {
                stat: BlockStorageStat {
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
                },

                dev: "sda".to_string(),
                size: 3907029168,
                maj: 8,
                min: 1,
                block_size: 4096,
            },
            partitions: vec![],
        };

        assert_eq!(BlockStorageStat::from_stat(SYS_BLOCK_DEV_STAT), Ok(dev.info.stat))
    }
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
