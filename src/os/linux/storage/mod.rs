mod types;

pub use types::*;

#[cfg(test)]
use super::mocks::SYS_BLOCK_DEV_STAT;
use super::{Error, SysPath};
use crate::util::{next, trim_parse_map};
use libc::{fileno, fopen, ioctl, size_t};
use std::ffi::{CStr, CString};
use std::fs;
use std::path::PathBuf;
use std::str::SplitAsciiWhitespace;

// Parses out major and minor number from str like '8:1'
// and returns a tuple (8, 1)
pub(crate) fn parse_maj_min(dev: &str) -> Option<(u32, u32)> {
    let mut elems = dev.split(':');

    if let Some(maj) = elems.next() {
        if let Some(min) = elems.next() {
            if let Ok(maj) = maj.trim().parse::<u32>() {
                if let Ok(min) = min.trim().parse::<u32>() {
                    return Some((maj, min));
                }
            }
        }
    }

    None
}

// FFI stuff

const _IOC_NRBITS: u64 = 8;
const _IOC_TYPEBITS: u64 = 8;
const _IOC_SIZEBITS: u64 = 14;
const _IOC_DIRBITS: u64 = 2;

const _IOC_NRMASK: u64 = (1 << _IOC_NRBITS) - 1;
const _IOC_TYPEMASK: u64 = (1 << _IOC_TYPEBITS) - 1;
const _IOC_SIZEMASK: u64 = (1 << _IOC_SIZEBITS) - 1;
const _IOC_DIRMASK: u64 = (1 << _IOC_DIRBITS) - 1;

const _IOC_NRSHIFT: u64 = 0;
const _IOC_TYPESHIFT: u64 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u64 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u64 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

const _IOC_NONE: u64 = 0;
const _IOC_WRITE: u64 = 1;
const _IOC_READ: u64 = 2;

#[allow(non_snake_case)]
const fn _IOC<T: Sized>(dir: u64, ty: u64, nr: u64) -> u64 {
    ((dir) << _IOC_DIRSHIFT)
        | ((ty) << _IOC_TYPESHIFT)
        | ((nr) << _IOC_NRSHIFT)
        | (std::mem::size_of::<T>() << _IOC_SIZESHIFT) as u64
}

#[allow(non_snake_case)]
const fn _IOR<T: Sized>(ty: u64, nr: u64) -> u64 {
    _IOC::<T>(_IOC_READ, ty, nr)
}

const BLKBSZGET: u64 = _IOR::<size_t>(0x12, 112);

pub(crate) fn blk_bsz_get(path: &str) -> Result<i64, Error> {
    let mut size: usize = 0;
    let path = CString::new(path).map_err(|e| Error::InvalidInputError(path.to_string(), e.to_string()))?;
    let mode = CStr::from_bytes_with_nul(b"r\0").unwrap();

    let f = unsafe { fopen(path.as_ptr(), mode.as_ptr()) };
    if f.is_null() {
        return Err(Error::FileReadError(
            path.to_string_lossy().to_string(),
            "failed to get file descriptor from `fopen`".to_string(),
        ));
    }

    unsafe { ioctl(fileno(f), BLKBSZGET, &mut size) };

    Ok(size as i64)
}

fn find_subdevices<T: FromSysPath<T>>(
    mut device_path: PathBuf,
    holder_or_slave: Hierarchy,
    dev_ty: DevType,
    hierarchy: bool,
) -> Option<Vec<T>> {
    match holder_or_slave {
        Hierarchy::Holders => device_path.push("holders"),
        Hierarchy::Slaves => device_path.push("slaves"),
        Hierarchy::None => {}
    };

    let mut devs = Vec::new();
    let prefix = dev_ty.prefix();
    if let Ok(dir) = fs::read_dir(device_path.as_path()) {
        for entry in dir {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(prefix) {
                        if let Ok(dev) = T::from_sys_path(device_path.join(name), hierarchy) {
                            devs.push(dev);
                        }
                    }
                }
            }
        }
        if devs.len() != 0 {
            return Some(devs);
        }
    }

    None
}

/// Returns block size of device in bytes
/// device argument must be a path to block device file descriptor
pub fn block_size(device: &str) -> Result<i64, Error> {
    blk_bsz_get(SysPath::Dev(device).path().to_string_lossy().as_ref())
}

pub fn stat_block_device(name: &str) -> Result<StorageDevice, Error> {
    StorageDevice::from_sys(name)
}

pub fn stat_device_mapper(name: &str) -> Result<DeviceMapper, Error> {
    DeviceMapper::from_sys(name)
}

pub fn stat_scsi_cdrom(name: &str) -> Result<ScsiCdrom, Error> {
    ScsiCdrom::from_sys(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_block_device_stat_from_sys_block_dev_stat() {
        let dev = StorageDevice {
            model: "ST2000DM008-2FR1".to_string(),
            vendor: "ATA".to_string(),
            state: "running".to_string(),
            info: BlockStorageInfo {
                stat: BlockStorageStat {
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
                },

                dev: "sda".to_string(),
                size: 3907029168,
                maj: 8,
                min: 1,
                block_size: 4096,
            },
            partitions: vec![],
        };

        assert_eq!(BlockStorageStat::from_stat(SYS_BLOCK_DEV_STAT), Ok(dev.info.stat))
    }
    #[test]
    fn parses_maj_min() {
        assert_eq!(parse_maj_min("X:5"), None);
        assert_eq!(parse_maj_min("1:Y"), None);
        assert_eq!(parse_maj_min("rand:"), None);
        assert_eq!(parse_maj_min(":xx"), None);
        assert_eq!(parse_maj_min("xxx"), None);
        assert_eq!(parse_maj_min("8:1"), Some((8, 1)))
    }
}
