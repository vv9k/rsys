use super::{block_size, parse_maj_min, SysPath};
use crate::{
    util::{next, trim_parse_map},
    Error, Result,
};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, str::SplitAsciiWhitespace};

/// Multiple partitions
pub type Partitions = Vec<Partition>;
/// Multiple device mappers
pub type DeviceMappers = Vec<DeviceMapper>;
/// Multiple storage devices
pub type StorageDevices = Vec<StorageDevice>;
/// Multiple scsi cdroms
pub type ScsiCdroms = Vec<ScsiCdrom>;
/// Multiple device storages
pub type MultipleDeviceStorages = Vec<MultipleDeviceStorage>;

/// Helper trait for generic parsing of storage devices from /sys/block/<dev>.
/// parse_stat decides whether or not to parse stats for slave/holder devices.
pub(crate) trait FromSysPath<T> {
    fn from_sys_path(path: PathBuf, hierarchy: bool, parse_stat: bool) -> Result<T>;
}

pub trait FromSysName<T> {
    fn from_sys(name: &str, parse_stat: bool) -> Result<T>;
}

pub(crate) fn stat<T: FromSysName<T>>(name: &str, parse_stat: bool) -> Result<T> {
    T::from_sys(name, parse_stat)
}

pub trait BlockStorageDeviceName {
    fn prefix() -> &'static str;
}

fn find_subdevices<T: FromSysPath<T> + BlockStorageDeviceName>(
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
    let prefix = T::prefix();
    if let Ok(dir) = fs::read_dir(device_path.as_path()) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(prefix) {
                        if let Ok(dev) = T::from_sys_path(device_path.join(name), hierarchy, parse_stat) {
                            devs.push(dev);
                        }
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Helper enum for generic function find_subdevices based on which
/// it decides to parse slaves or holders or nothing.
pub(crate) enum Hierarchy {
    Holders,
    Slaves,
    None,
}

#[derive(Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Provides several statistics about the state of block device.
/// This information is gathered from /sys/block/<dev>/stat
pub struct BlockStorageStat {
    /// Count of read I/O requests
    pub read_ios: usize,
    /// Count of merged read I/O requests
    pub read_merges: usize,
    /// Count of sectors read from this block device. One sector is 512-bytes big.
    pub read_sectors: usize,
    /// Milliseconds that I/O requests have waited on this block device
    /// while processing read requests.
    pub read_ticks: u64,
    /// Count of write I/O requests
    pub write_ios: usize,
    /// Count of merged write I/O requests
    pub write_merges: usize,
    /// Count of sectors written to this block device. One sector is 512-bytes big.
    pub write_sectors: usize,
    /// Count of milliseconds that I/O requests have waited on this block device
    /// while processing write requests.
    pub write_ticks: u64,
    /// Number of I/O requests that have been issued to the device driver
    /// but have not yet completed.
    pub in_flight: usize,
    /// Count of milliseconds that this device had I/O requests queued for.
    pub io_ticks: u64,
    pub time_in_queue: u64,
    /// Count of discard I/O requests
    pub discard_ios: usize,
    /// Count of merged discard I/O requests
    pub discard_merges: usize,
    /// Count of sectors discarded from this block device. One sector is 512-bytes big.
    pub discard_sectors: usize,
    /// Count of milliseconds that I/O requests have waited on this block device
    /// while processing discard requests.
    pub discard_ticks: u64,
}
impl BlockStorageStat {
    pub(crate) fn from_stat(stat: &str) -> Result<BlockStorageStat> {
        let mut elems = stat.split_ascii_whitespace();

        Ok(BlockStorageStat {
            read_ios: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            read_merges: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            read_sectors: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            read_ticks: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            write_ios: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            write_merges: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            write_sectors: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            write_ticks: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            in_flight: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            io_ticks: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            time_in_queue: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            discard_ios: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            discard_merges: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            discard_sectors: next::<usize, SplitAsciiWhitespace>(&mut elems, &stat)?,
            discard_ticks: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
        })
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
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
        let (maj, min) = parse_maj_min(&SysPath::Custom(path.join("dev")).read()?).unwrap_or_default();
        let device = path
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
            size: trim_parse_map::<usize>(&SysPath::Custom(path.join("size")).read()?)?,
            maj,
            min,
            block_size: block_size(&device)?,
            stat: if parse_stat {
                Some(BlockStorageStat::from_stat(
                    &SysPath::Custom(path.join("stat")).read()?,
                )?)
            } else {
                None
            },
            path,
        })
    }
    pub fn update_stats(&mut self) -> Result<()> {
        self.stat = Some(BlockStorageStat::from_stat(
            &SysPath::Custom(self.path.join("stat")).read()?,
        )?);
        Ok(())
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a scsi cdrom
pub struct ScsiCdrom {
    pub info: BlockStorageInfo,
    pub model: String,
    pub vendor: String,
    pub state: String,
}
impl FromSysName<ScsiCdrom> for ScsiCdrom {
    fn from_sys(name: &str, _parse_stat: bool) -> Result<ScsiCdrom> {
        if !name.starts_with("sr") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "SCSI CDrom device name must begin with 'sr'".to_string(),
            ));
        }
        Ok(ScsiCdrom {
            info: BlockStorageInfo::from_sys_path(SysPath::SysBlockDev(name).path(), true)?,
            model: trim_parse_map::<String>(&SysPath::SysBlockDevModel(name).read()?)?,
            vendor: trim_parse_map::<String>(&SysPath::SysBlockDevVendor(name).read()?)?,
            state: trim_parse_map::<String>(&SysPath::SysBlockDevState(name).read()?)?,
        })
    }
}
impl BlockStorageDeviceName for ScsiCdrom {
    fn prefix() -> &'static str {
        "sr"
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a block storage device.
pub struct StorageDevice {
    pub info: BlockStorageInfo,
    pub model: String,
    pub vendor: String,
    pub state: String,
    pub partitions: Partitions,
}
impl FromSysName<StorageDevice> for StorageDevice {
    fn from_sys(name: &str, parse_stat: bool) -> Result<StorageDevice> {
        if !name.starts_with("sd") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "block storage device name must begin with 'sd'".to_string(),
            ));
        }
        Ok(StorageDevice {
            info: BlockStorageInfo::from_sys_path(SysPath::SysBlockDev(name).path(), true)?,
            model: trim_parse_map::<String>(&SysPath::SysBlockDevModel(name).read()?)?,
            vendor: trim_parse_map::<String>(&SysPath::SysBlockDevVendor(name).read()?)?,
            state: trim_parse_map::<String>(&SysPath::SysBlockDevState(name).read()?)?,
            partitions: find_subdevices::<Partition>(
                SysPath::SysBlockDev(name).path(),
                Hierarchy::None,
                false,
                parse_stat,
            )
            .unwrap_or_default(),
        })
    }
}
impl BlockStorageDeviceName for StorageDevice {
    fn prefix() -> &'static str {
        "sd"
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct DeviceMapper {
    pub info: BlockStorageInfo,
    pub name: String,
    pub uuid: String,
    pub slave_parts: Option<Partitions>,
    pub slave_mds: Option<MultipleDeviceStorages>,
}
impl FromSysName<DeviceMapper> for DeviceMapper {
    fn from_sys(name: &str, parse_stat: bool) -> Result<DeviceMapper> {
        if !name.starts_with("dm") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "device mapper name must begin with 'dm'".to_string(),
            ));
        }
        Ok(DeviceMapper {
            info: BlockStorageInfo::from_sys_path(SysPath::SysBlockDev(name).path(), true)?,
            uuid: trim_parse_map::<String>(&SysPath::SysDevMapperUuid(name).read()?)?,
            name: trim_parse_map::<String>(&SysPath::SysDevMapperName(name).read()?)?,
            slave_parts: find_subdevices::<Partition>(
                SysPath::SysBlockDev(name).path(),
                Hierarchy::Slaves,
                false,
                parse_stat,
            ),
            slave_mds: find_subdevices::<MultipleDeviceStorage>(
                SysPath::SysBlockDev(name).path(),
                Hierarchy::Slaves,
                true,
                parse_stat,
            ),
        })
    }
}

impl FromSysPath<DeviceMapper> for DeviceMapper {
    fn from_sys_path(path: PathBuf, hierarchy: bool, parse_stat: bool) -> Result<Self> {
        Ok(DeviceMapper {
            info: BlockStorageInfo::from_sys_path(path.clone(), parse_stat)?,
            name: trim_parse_map::<String>(&SysPath::Custom(path.join("dm").join("name")).read()?)?,
            uuid: trim_parse_map::<String>(&SysPath::Custom(path.join("dm").join("uuid")).read()?)?,
            slave_mds: if hierarchy {
                find_subdevices::<MultipleDeviceStorage>(path.clone(), Hierarchy::Slaves, true, parse_stat)
            } else {
                None
            },
            slave_parts: if hierarchy {
                find_subdevices::<Partition>(path.clone(), Hierarchy::Slaves, false, parse_stat)
            } else {
                None
            },
        })
    }
}
impl BlockStorageDeviceName for DeviceMapper {
    fn prefix() -> &'static str {
        "dm"
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Partition {
    pub info: BlockStorageInfo,
    pub holder_mds: Option<MultipleDeviceStorages>,
    pub holder_dms: Option<DeviceMappers>,
}
impl FromSysPath<Partition> for Partition {
    fn from_sys_path(path: PathBuf, hierarchy: bool, parse_stat: bool) -> Result<Self> {
        Ok(Partition {
            info: BlockStorageInfo::from_sys_path(path.clone(), parse_stat)?,
            holder_mds: if hierarchy {
                find_subdevices::<MultipleDeviceStorage>(path.clone(), Hierarchy::Holders, false, parse_stat)
            } else {
                None
            },
            holder_dms: if hierarchy {
                find_subdevices::<DeviceMapper>(path, Hierarchy::Holders, false, parse_stat)
            } else {
                None
            },
        })
    }
}
impl BlockStorageDeviceName for Partition {
    fn prefix() -> &'static str {
        "sd"
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct MultipleDeviceStorage {
    pub info: BlockStorageInfo,
    pub level: String,
    pub slave_parts: Option<Partitions>,
    pub holder_devices: Option<DeviceMappers>,
}
impl MultipleDeviceStorage {
    pub(crate) fn from_sys_path(path: PathBuf, hierarchy: bool, parse_stat: bool) -> Result<MultipleDeviceStorage> {
        Ok(MultipleDeviceStorage {
            info: BlockStorageInfo::from_sys_path(path.clone(), parse_stat)?,
            level: trim_parse_map::<String>(&SysPath::Custom(path.join("md").join("level")).read()?)?,
            slave_parts: if hierarchy {
                find_subdevices::<Partition>(path.clone(), Hierarchy::Slaves, false, parse_stat)
            } else {
                None
            },
            holder_devices: if hierarchy {
                find_subdevices::<DeviceMapper>(path.clone(), Hierarchy::Holders, false, parse_stat)
            } else {
                None
            },
        })
    }
}
impl FromSysName<MultipleDeviceStorage> for MultipleDeviceStorage {
    fn from_sys(name: &str, parse_stat: bool) -> Result<MultipleDeviceStorage> {
        if !name.starts_with("md") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "multiple device storage name must begin with 'md'".to_string(),
            ));
        }
        MultipleDeviceStorage::from_sys_path(SysPath::SysClassBlockDev(name).path(), true, parse_stat)
    }
}
impl FromSysPath<MultipleDeviceStorage> for MultipleDeviceStorage {
    fn from_sys_path(path: PathBuf, hierarchy: bool, parse_stat: bool) -> Result<Self> {
        Self::from_sys_path(path, hierarchy, parse_stat)
    }
}
impl BlockStorageDeviceName for MultipleDeviceStorage {
    fn prefix() -> &'static str {
        "md"
    }
}
