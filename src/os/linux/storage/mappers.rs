use crate::linux::storage::{
    find_subdevices, BlockStorageDeviceName, BlockStorageInfo, FromSysName, FromSysPath, Hierarchy,
    MultipleDeviceStorage, MultipleDeviceStorages, Partitions, _find_subdevices,
};
use crate::linux::SysFs;
use crate::{util::trim_parse_map, Error, Result};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Multiple device mappers
pub type DeviceMappers = Vec<DeviceMapper>;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct DeviceMapper {
    pub info: BlockStorageInfo,
    pub name: String,
    pub uuid: String,
    pub slave_parts: Option<Partitions>,
    pub slave_mds: Option<MultipleDeviceStorages>,
}

impl BlockStorageDeviceName for DeviceMapper {
    fn prefix() -> &'static str {
        "dm"
    }
}

impl FromSysName<DeviceMapper> for DeviceMapper {
    fn from_sys(name: &str, parse_stat: bool) -> Result<DeviceMapper> {
        if !name.starts_with("dm") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "device mapper name must begin with 'dm'".to_string(),
            ));
        }
        let base_path = SysFs::Sys.join("block").join("name");
        let base_pathbuf = base_path.clone().path();

        Ok(DeviceMapper {
            info: BlockStorageInfo::from_sys_path(base_pathbuf.clone(), true)?,
            uuid: trim_parse_map::<String>(&base_path.clone().join("dm").join("uuid").read()?)?,
            name: trim_parse_map::<String>(&base_path.join("dm").join("name").read()?)?,
            slave_parts: _find_subdevices(None, base_pathbuf.clone(), Hierarchy::Slaves, false, parse_stat),
            slave_mds: find_subdevices::<MultipleDeviceStorage>(base_pathbuf, Hierarchy::Slaves, true, parse_stat),
        })
    }
}

impl FromSysPath<DeviceMapper> for DeviceMapper {
    fn from_sys_path(path: PathBuf, hierarchy: bool, parse_stat: bool) -> Result<Self> {
        Ok(DeviceMapper {
            info: BlockStorageInfo::from_sys_path(path.clone(), parse_stat)?,
            name: trim_parse_map::<String>(&SysFs::Custom(path.clone()).join("dm").join("name").read()?)?,
            uuid: trim_parse_map::<String>(&SysFs::Custom(path.clone()).join("dm").join("uuid").read()?)?,
            slave_mds: if hierarchy {
                find_subdevices::<MultipleDeviceStorage>(path.clone(), Hierarchy::Slaves, true, parse_stat)
            } else {
                None
            },
            slave_parts: if hierarchy {
                _find_subdevices(None, path, Hierarchy::Slaves, false, parse_stat)
            } else {
                None
            },
        })
    }
}
