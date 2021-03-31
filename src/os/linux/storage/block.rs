use crate::linux::storage::{
    BlockStorageDeviceName, BlockStorageInfo, FromSysName, Hierarchy, Partitions, _find_subdevices,
};
use crate::linux::SysFs;
use crate::{util::trim_parse_map, Error, Result};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Multiple storage devices
pub type StorageDevices = Vec<StorageDevice>;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a block storage device.
pub struct StorageDevice {
    pub info: BlockStorageInfo,
    pub model: String,
    pub vendor: String,
    pub state: String,
    pub partitions: Partitions,
}

impl BlockStorageDeviceName for StorageDevice {
    fn prefix() -> &'static str {
        "sd"
    }
}

impl FromSysName<StorageDevice> for StorageDevice {
    fn from_sys(name: &str, parse_stat: bool) -> Result<Self> {
        if !name.starts_with(Self::prefix()) {
            return Err(Error::InvalidInputError(
                name.to_string(),
                format!("block storage device name must begin with '{}'", Self::prefix()),
            ));
        }
        let base_path = SysFs::Sys.join("block").join(name);
        let dev_path = base_path.clone().join("device");

        Ok(Self {
            info: BlockStorageInfo::from_sys_path(base_path.path(), true)?,
            model: trim_parse_map::<String>(&dev_path.clone().join("model").read()?)?,
            vendor: trim_parse_map::<String>(&dev_path.clone().join("vendor").read()?)?,
            state: trim_parse_map::<String>(&dev_path.join("state").read()?)?,
            partitions: _find_subdevices(
                None,
                SysFs::Sys.join("block").join(name).path(),
                Hierarchy::None,
                false,
                parse_stat,
            )
            .unwrap_or_default(),
        })
    }
}
