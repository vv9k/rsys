use crate::{Error, Result};
use nix::sys::statfs::statfs;

pub(crate) fn blk_bsz_get(path: &str) -> Result<i64> {
    statfs(path)
        .map_err(|e| Error::FfiError(format!("getting block size of `{}`", path), e.to_string()))
        .map(|v| v.block_size())
}
