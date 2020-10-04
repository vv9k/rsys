use crate::{Error, Result};
use libc::{fileno, fopen, ioctl, size_t};
use std::ffi::{CStr, CString};

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

#[cfg(not(target_arch = "arm"))]
pub(crate) fn blk_bsz_get(path: &str) -> Result<i64> {
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

#[cfg(target_arch = "arm")]
pub(crate) fn blk_bsz_get(path: &str) -> Result<i64> {
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

    unsafe { ioctl(fileno(f), BLKBSZGET as u32, &mut size) };

    Ok(size as i64)
}
