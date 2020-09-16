#[cfg(test)]
use super::mocks::SYS_BLOCK_DEV_STAT;
use super::{block_size, next, parse_maj_min, trim_parse_map, Error, SysPath};
use std::fs;
use std::path::PathBuf;
use std::str::SplitAsciiWhitespace;

pub type Partitions = Vec<Partition>;

trait FromSysPath<T> {
    fn from_sys_path(path: PathBuf, hierarchy: bool) -> Result<T, Error>;
}

#[derive(Clone, Debug)]
enum Hierarchy {
    Holders,
    Slaves,
    None,
}

#[derive(Clone, Debug)]
enum DevType {
    Partition,
    DevMapper,
    Md,
}
impl DevType {
    fn prefix(self) -> &'static str {
        match self {
            DevType::Partition => "sd",
            DevType::DevMapper => "dm",
            DevType::Md => "md",
        }
    }
}

fn find_subdevices<T: FromSysPath<T>>(
    mut device_path: PathBuf,
    holder_or_slave: Hierarchy,
    dev_ty: DevType,
    hierarchy: bool,
) -> Option<Vec<T>> {
    match holder_or_slave {
        Hierarchy::Holders => device_path.push("holders"),
        Hierarchy::Slaves => device_path.push("slaves"),
        Hierarchy::None => {}
    };

    let mut devs = Vec::new();
    let prefix = dev_ty.prefix();
    if let Ok(dir) = fs::read_dir(device_path.as_path()) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(prefix) {
                        if let Ok(dev) = T::from_sys_path(device_path.join(name), hierarchy) {
                            devs.push(dev);
                        }
                    }
                }
            }
        }
        if devs.len() != 0 {
            return Some(devs);
        }
    }

    None
}

#[derive(Debug, Eq, PartialEq)]
/// Represents stats of a block storage device
/// read from /sys/block/<device>/stat
pub struct BlockStorageStat {
    pub read_ios: usize,
    pub read_merges: usize,
    pub read_sectors: usize,
    pub read_ticks: u64,
    pub write_ios: usize,
    pub write_merges: usize,
    pub write_sectors: usize,
    pub write_ticks: u64,
    pub in_flight: usize,
    pub io_ticks: u64,
    pub time_in_queue: u64,
    pub discard_ios: usize,
    pub discard_merges: usize,
    pub discard_sectors: usize,
    pub discard_ticks: u64,
}
impl BlockStorageStat {
    pub(crate) fn from_stat(stat: &str) -> Result<BlockStorageStat, Error> {
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

#[derive(Debug, Eq, PartialEq)]
pub struct BlockStorageInfo {
    pub dev: String,
    pub size: usize,
    pub maj: u32,
    pub min: u32,
    pub block_size: i64,
    pub stat: BlockStorageStat,
}
impl BlockStorageInfo {
    fn from_sys_path(path: PathBuf) -> Result<BlockStorageInfo, Error> {
        let (maj, min) =
            parse_maj_min(&SysPath::Custom(path.join("dev").to_string_lossy().to_string()).read()?).unwrap_or_default();
        let device = path
            .file_name()
            .ok_or(Error::InvalidInputError(
                path.to_string_lossy().to_string(),
                "Given path doesn't have a file name".to_string(),
            ))?
            .to_string_lossy()
            .to_string();
        Ok(BlockStorageInfo {
            dev: device.clone(),
            size: trim_parse_map::<usize>(&SysPath::Custom(path.join("size").to_string_lossy().to_string()).read()?)?,
            maj,
            min,
            block_size: block_size(&device)?,
            stat: BlockStorageStat::from_stat(
                &SysPath::Custom(path.join("stat").to_string_lossy().to_string()).read()?,
            )?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ScsiCdrom {
    pub info: BlockStorageInfo,
    pub model: String,
    pub vendor: String,
    pub state: String,
}
impl ScsiCdrom {
    pub(crate) fn from_sys(name: &str) -> Result<ScsiCdrom, Error> {
        if !name.starts_with("sr") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "SCSI CDrom device name must begin with 'sr'".to_string(),
            ));
        }
        Ok(ScsiCdrom {
            info: BlockStorageInfo::from_sys_path(SysPath::SysBlockDev(name).path())?,
            model: trim_parse_map::<String>(&SysPath::SysBlockDevModel(name).read()?)?,
            vendor: trim_parse_map::<String>(&SysPath::SysBlockDevVendor(name).read()?)?,
            state: trim_parse_map::<String>(&SysPath::SysBlockDevState(name).read()?)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Represents a block storage device.
pub struct StorageDevice {
    pub info: BlockStorageInfo,
    pub model: String,
    pub vendor: String,
    pub state: String,
    pub partitions: Partitions,
}
impl StorageDevice {
    pub(crate) fn from_sys(name: &str) -> Result<StorageDevice, Error> {
        if !name.starts_with("sd") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "block storage device name must begin with 'sd'".to_string(),
            ));
        }
        Ok(StorageDevice {
            info: BlockStorageInfo::from_sys_path(SysPath::SysBlockDev(name).path())?,
            model: trim_parse_map::<String>(&SysPath::SysBlockDevModel(name).read()?)?,
            vendor: trim_parse_map::<String>(&SysPath::SysBlockDevVendor(name).read()?)?,
            state: trim_parse_map::<String>(&SysPath::SysBlockDevState(name).read()?)?,
            partitions: find_subdevices::<Partition>(
                SysPath::SysBlockDev(name).path(),
                Hierarchy::None,
                DevType::Partition,
                false,
            )
            .unwrap_or_default(),
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DeviceMapper {
    pub info: BlockStorageInfo,
    pub name: String,
    pub uuid: String,
    pub slave_parts: Option<Partitions>,
    pub slave_mds: Option<Vec<MultipleDeviceStorage>>,
}
impl DeviceMapper {
    pub(crate) fn from_sys(name: &str) -> Result<DeviceMapper, Error> {
        if !name.starts_with("dm") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "device mapper name must begin with 'dm'".to_string(),
            ));
        }
        Ok(DeviceMapper {
            info: BlockStorageInfo::from_sys_path(SysPath::SysBlockDev(name).path())?,
            uuid: trim_parse_map::<String>(&SysPath::SysDevMapperUuid(name).read()?)?,
            name: trim_parse_map::<String>(&SysPath::SysDevMapperName(name).read()?)?,
            slave_parts: find_subdevices::<Partition>(
                SysPath::SysBlockDev(name).path(),
                Hierarchy::Slaves,
                DevType::Partition,
                false,
            ),
            slave_mds: find_subdevices::<MultipleDeviceStorage>(
                SysPath::SysBlockDev(name).path(),
                Hierarchy::Slaves,
                DevType::Md,
                true,
            ),
        })
    }
}

impl FromSysPath<DeviceMapper> for DeviceMapper {
    fn from_sys_path(path: PathBuf, hierarchy: bool) -> Result<Self, Error> {
        Ok(DeviceMapper {
            info: BlockStorageInfo::from_sys_path(path.clone())?,
            name: trim_parse_map::<String>(
                &SysPath::Custom(path.join("dm").join("name").to_string_lossy().to_string()).read()?,
            )?,
            uuid: trim_parse_map::<String>(
                &SysPath::Custom(path.join("dm").join("uuid").to_string_lossy().to_string()).read()?,
            )?,
            slave_mds: if hierarchy {
                find_subdevices::<MultipleDeviceStorage>(path.clone(), Hierarchy::Slaves, DevType::Md, true)
            } else {
                None
            },
            slave_parts: if hierarchy {
                find_subdevices::<Partition>(path.clone(), Hierarchy::Slaves, DevType::Partition, false)
            } else {
                None
            },
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Partition {
    pub info: BlockStorageInfo,
    pub holder_mds: Option<Vec<MultipleDeviceStorage>>,
    pub holder_dms: Option<Vec<DeviceMapper>>,
}
impl FromSysPath<Partition> for Partition {
    fn from_sys_path(path: PathBuf, hierarchy: bool) -> Result<Self, Error> {
        Ok(Partition {
            info: BlockStorageInfo::from_sys_path(path.clone())?,
            holder_mds: if hierarchy {
                find_subdevices::<MultipleDeviceStorage>(path.clone(), Hierarchy::Holders, DevType::Md, false)
            } else {
                None
            },
            holder_dms: if hierarchy {
                find_subdevices::<DeviceMapper>(path.clone(), Hierarchy::Holders, DevType::DevMapper, false)
            } else {
                None
            },
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct MultipleDeviceStorage {
    pub info: BlockStorageInfo,
    pub level: String,
    pub slave_parts: Option<Partitions>,
}
impl MultipleDeviceStorage {
    pub(crate) fn from_sys_path(path: PathBuf, hierarchy: bool) -> Result<MultipleDeviceStorage, Error> {
        Ok(MultipleDeviceStorage {
            info: BlockStorageInfo::from_sys_path(path.clone())?,
            level: trim_parse_map::<String>(
                &SysPath::Custom(path.join("md").join("level").to_string_lossy().to_string()).read()?,
            )?,
            slave_parts: if hierarchy {
                find_subdevices::<Partition>(path.clone(), Hierarchy::Slaves, DevType::Partition, false)
            } else {
                None
            },
        })
    }
}
impl FromSysPath<MultipleDeviceStorage> for MultipleDeviceStorage {
    fn from_sys_path(path: PathBuf, hierarchy: bool) -> Result<Self, Error> {
        Self::from_sys_path(path, hierarchy)
    }
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
}
