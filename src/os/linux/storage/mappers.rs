use crate::linux::storage::{
    find_subdevices, BlockStorageDeviceName, BlockStorageInfo, FromSysName, FromSysPath, Hierarchy,
    MultipleDeviceStorage, MultipleDeviceStorages, Partitions, _find_subdevices,
};
use crate::linux::{SysFs, SysPath};
use crate::{util::trim_parse_map, Error, Result};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

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
    fn from_sys(name: &str, parse_stat: bool) -> Result<Self> {
        if !name.starts_with(Self::prefix()) {
            return Err(Error::InvalidInputError(
                name.to_string(),
                format!("device mapper name must begin with '{}'", Self::prefix()),
            ));
        }
        let base_path = SysFs::Sys.join("block/name");

        Ok(Self {
            info: BlockStorageInfo::from_sys_path(&base_path, true)?,
            uuid: trim_parse_map::<String>(&base_path.extend("dm/uuid").read()?)?,
            name: trim_parse_map::<String>(&base_path.extend("dm/name").read()?)?,
            slave_parts: _find_subdevices(None, &base_path, Hierarchy::Slaves, false, parse_stat),
            slave_mds: find_subdevices::<MultipleDeviceStorage>(&base_path, Hierarchy::Slaves, true, parse_stat),
        })
    }
}

impl FromSysPath<DeviceMapper> for DeviceMapper {
    fn from_sys_path(path: &SysPath, hierarchy: bool, parse_stat: bool) -> Result<Self> {
        Ok(Self {
            info: BlockStorageInfo::from_sys_path(path, parse_stat)?,
            name: trim_parse_map::<String>(&path.extend("dm/name").read()?)?,
            uuid: trim_parse_map::<String>(&path.extend("dm/uuid").read()?)?,
            slave_mds: if hierarchy {
                find_subdevices::<MultipleDeviceStorage>(path, Hierarchy::Slaves, true, parse_stat)
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
