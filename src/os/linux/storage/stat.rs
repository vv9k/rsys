use crate::{util::next, Result};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::str::SplitAsciiWhitespace;

#[derive(Clone, Default, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Provides several statistics about the state of block device.
/// This information is gathered from /sys/block/<dev>/stat
pub struct BlockStorageStat {
    /// Count of read I/O requests
    pub read_ios: usize,
    /// Count of merged read I/O requests
    pub read_merges: usize,
    /// Count of sectors read from this block device. One sector is 512-bytes big.
    pub read_sectors: usize,
    /// Milliseconds that I/O requests have waited on this block device
    /// while processing read requests.
    pub read_ticks: u64,
    /// Count of write I/O requests
    pub write_ios: usize,
    /// Count of merged write I/O requests
    pub write_merges: usize,
    /// Count of sectors written to this block device. One sector is 512-bytes big.
    pub write_sectors: usize,
    /// Count of milliseconds that I/O requests have waited on this block device
    /// while processing write requests.
    pub write_ticks: u64,
    /// Number of I/O requests that have been issued to the device driver
    /// but have not yet completed.
    pub in_flight: usize,
    /// Count of milliseconds that this device had I/O requests queued for.
    pub io_ticks: u64,
    pub time_in_queue: u64,
    /// Count of discard I/O requests
    pub discard_ios: usize,
    /// Count of merged discard I/O requests
    pub discard_merges: usize,
    /// Count of sectors discarded from this block device. One sector is 512-bytes big.
    pub discard_sectors: usize,
    /// Count of milliseconds that I/O requests have waited on this block device
    /// while processing discard requests.
    pub discard_ticks: u64,
}

impl BlockStorageStat {
    pub(crate) fn from_stat(stat: &str) -> Result<BlockStorageStat> {
        let mut elems = stat.split_ascii_whitespace();

        macro_rules! _next {
            ($t:tt) => {
                next::<$t, SplitAsciiWhitespace>(&mut elems, &stat)?
            };
        }

        Ok(BlockStorageStat {
            read_ios: _next!(usize),
            read_merges: _next!(usize),
            read_sectors: _next!(usize),
            read_ticks: _next!(u64),
            write_ios: _next!(usize),
            write_merges: _next!(usize),
            write_sectors: _next!(usize),
            write_ticks: _next!(u64),
            in_flight: _next!(usize),
            io_ticks: _next!(u64),
            time_in_queue: _next!(u64),
            discard_ios: _next!(usize),
            discard_merges: _next!(usize),
            discard_sectors: _next!(usize),
            discard_ticks: _next!(u64),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linux::mocks::SYS_BLOCK_DEV_STAT;
    use crate::linux::storage::BlockStorageInfo;
    use crate::linux::SysFs;
    use std::{fs, io};

    #[test]
    fn parses_block_storage_device_stat() {
        let stat = BlockStorageStat {
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
        };

        assert_eq!(BlockStorageStat::from_stat(SYS_BLOCK_DEV_STAT), Ok(stat))
    }
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

        let p = SysFs::Custom(p).into_syspath();

        assert_eq!(Ok(info.clone()), BlockStorageInfo::from_sys_path(&p, true));

        info.stat = None;

        assert_eq!(Ok(info), BlockStorageInfo::from_sys_path(&p, false));

        dir.close()
    }
}
