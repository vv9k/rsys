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
    fn from_sys(name: &str, _parse_stat: bool) -> Result<Self> {
        if !name.starts_with(Self::prefix()) {
            return Err(Error::InvalidInputError(
                name.to_string(),
                format!("SCSI CDrom device name must begin with '{}'", Self::prefix()),
            ));
        }
        let base_path = SysFs::Sys.join("block").join(name);
        let dev_path = base_path.extend("device");

        Ok(Self {
            info: BlockStorageInfo::from_sys_path(&base_path, true)?,
            model: trim_parse_map::<String>(&dev_path.extend("model").read()?)?,
            vendor: trim_parse_map::<String>(&dev_path.extend("vendor").read()?)?,
            state: trim_parse_map::<String>(&dev_path.join("state").read()?)?,
        })
    }
}
