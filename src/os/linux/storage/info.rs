use crate::linux::storage::{block_size, parse_maj_min, BlockStorageStat};
use crate::linux::SysFs;
use crate::{util::trim_parse_map, Error, Result};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
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
        let (maj, min) = parse_maj_min(&SysFs::Custom(path.clone()).join("dev").read()?).unwrap_or_default();
        let device = path
            .clone()
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
            size: trim_parse_map::<usize>(&SysFs::Custom(path.clone()).join("size").read()?)?,
            maj,
            min,
            block_size: block_size(&device)?,
            stat: if parse_stat {
                Some(BlockStorageStat::from_stat(
                    &SysFs::Custom(path.clone()).join("stat").read()?,
                )?)
            } else {
                None
            },
            path,
        })
    }
    pub fn update_stats(&mut self) -> Result<()> {
        self.stat = Some(BlockStorageStat::from_stat(
            &SysFs::Custom(self.path.clone()).join("stat").read()?,
        )?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linux::mocks::SYS_BLOCK_DEV_STAT;
    use std::{fs, io};

    #[test]
    fn parses_block_storage_info() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let p = dir.path().join("sda");

        fs::create_dir(p.as_path())?;

        fs::write(p.as_path().join("stat"), SYS_BLOCK_DEV_STAT)?;
        fs::write(p.as_path().join("dev"), b"8:0")?;
        fs::write(p.as_path().join("size"), b"3907029168")?;

        let mut info = BlockStorageInfo {
            dev: "sda".to_string(),
            // This probably shouldn't be here
            block_size: 4096,
            path: p.clone(),
            size: 3907029168,
            maj: 8,
            min: 0,
            stat: Some(BlockStorageStat {
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
            }),
        };

        assert_eq!(Ok(info.clone()), BlockStorageInfo::from_sys_path(p.clone(), true));

        info.stat = None;

        assert_eq!(Ok(info), BlockStorageInfo::from_sys_path(p, false));

        dir.close()
    }
}
