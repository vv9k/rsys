use crate::{Error, Result};
use nix::sys::statfs;

pub(crate) fn blk_bsz_get(path: &str) -> Result<i64> {
    statfs::statfs(path).map(|v| v.block_size() as i64).map_err(Error::from)
}
