use crate::linux::storage::{
    find_subdevices, BlockStorageInfo, DeviceMapper, DeviceMappers, FromSysPath, Hierarchy, MultipleDeviceStorage,
    MultipleDeviceStorages,
};
use crate::Result;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Multiple partitions
pub type Partitions = Vec<Partition>;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
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
