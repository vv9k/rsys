//! Other api
use super::SysPath;
use crate::Result;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a line of /proc/mounts
pub struct MountPoint {
    volume: String,
    path: String,
    voltype: String,
    options: String,
}
impl MountPoint {
    pub fn new(volume: &str, path: &str, voltype: &str, options: &str) -> MountPoint {
        MountPoint {
            volume: volume.to_string(),
            path: path.to_string(),
            voltype: voltype.to_string(),
            options: options.to_string(),
        }
    }
    pub(crate) fn from_line(line: &str) -> Option<MountPoint> {
        let mut elems = line.split_ascii_whitespace().take(4);
        if elems.clone().count() >= 4 {
            let volume = elems.next().unwrap();
            let path = elems.next().unwrap();
            let voltype = elems.next().unwrap();
            let options = elems.next().unwrap();
            return Some(Self::new(volume, path, voltype, options));
        }

        None
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct MountPoints(Vec<MountPoint>);

//--------------------------------------------------------------------------------
// API

/// Returns MountPoints read from /proc/mounts
pub fn mounts() -> Result<MountPoints> {
    let mut mps = Vec::new();
    for line in SysPath::ProcMounts.read()?.split('\n') {
        if let Some(mp) = MountPoint::from_line(line) {
            mps.push(mp);
        }
    }
    Ok(MountPoints(mps))
}
