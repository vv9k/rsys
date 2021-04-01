use crate::linux::storage::{
    _find_subdevices, find_subdevices, BlockStorageDeviceName, BlockStorageInfo, DeviceMapper, DeviceMappers,
    FromSysName, FromSysPath, Hierarchy, Partitions,
};
use crate::linux::{SysFs, SysPath};
use crate::{util::trim_parse_map, Error, Result};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Multiple device storages
pub type MultipleDeviceStorages = Vec<MultipleDeviceStorage>;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct MultipleDeviceStorage {
    pub info: BlockStorageInfo,
    pub level: String,
    pub slave_parts: Option<Partitions>,
    pub holder_devices: Option<DeviceMappers>,
}

impl BlockStorageDeviceName for MultipleDeviceStorage {
    fn prefix() -> &'static str {
        "md"
    }
}

impl FromSysName<MultipleDeviceStorage> for MultipleDeviceStorage {
    fn from_sys(name: &str, parse_stat: bool) -> Result<Self> {
        if !name.starts_with("md") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "multiple device storage name must begin with 'md'".to_string(),
            ));
        }
        Self::from_sys_path(&SysFs::Sys.join("class/block").join(name), true, parse_stat)
    }
}

impl FromSysPath<MultipleDeviceStorage> for MultipleDeviceStorage {
    fn from_sys_path(path: &SysPath, hierarchy: bool, parse_stat: bool) -> Result<Self> {
        Ok(Self {
            info: BlockStorageInfo::from_sys_path(&path, parse_stat)?,
            level: trim_parse_map::<String>(&path.extend("md/level").read()?)?,
            slave_parts: if hierarchy {
                _find_subdevices(None, &path, Hierarchy::Slaves, false, parse_stat)
            } else {
                None
            },
            holder_devices: if hierarchy {
                find_subdevices::<DeviceMapper>(path, Hierarchy::Holders, false, parse_stat)
            } else {
                None
            },
        })
    }
}
