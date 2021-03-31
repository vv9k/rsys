use crate::linux::storage::{BlockStorageDeviceName, BlockStorageInfo, FromSysName};
use crate::linux::SysFs;
use crate::{util::trim_parse_map, Error, Result};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

/// Multiple scsi cdroms
pub type ScsiCdroms = Vec<ScsiCdrom>;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a scsi cdrom
pub struct ScsiCdrom {
    pub info: BlockStorageInfo,
    pub model: String,
    pub vendor: String,
    pub state: String,
}

impl BlockStorageDeviceName for ScsiCdrom {
    fn prefix() -> &'static str {
        "sr"
    }
}

impl FromSysName<ScsiCdrom> for ScsiCdrom {
    fn from_sys(name: &str, _parse_stat: bool) -> Result<ScsiCdrom> {
        if !name.starts_with("sr") {
            return Err(Error::InvalidInputError(
                name.to_string(),
                "SCSI CDrom device name must begin with 'sr'".to_string(),
            ));
        }
        let base_path = SysFs::Sys.join("block").join(name);
        let dev_path = base_path.clone().join("device");

        Ok(ScsiCdrom {
            info: BlockStorageInfo::from_sys_path(base_path.path(), true)?,
            model: trim_parse_map::<String>(&dev_path.clone().join("model").read()?)?,
            vendor: trim_parse_map::<String>(&dev_path.clone().join("vendor").read()?)?,
            state: trim_parse_map::<String>(&dev_path.join("state").read()?)?,
        })
    }
}
