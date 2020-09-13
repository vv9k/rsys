use super::{run, Error, SysPath};
use libc::{fileno, fopen, ioctl, size_t};
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::process::Command;
use std::str::FromStr;

#[cfg(test)]
use super::mocks::*;

pub(crate) const MODEL_NAME: &str = "model name";
pub(crate) const CPU_CORES: &str = "cpu cores";
pub(crate) const SIBLINGS: &str = "siblings";
pub(crate) const CPU_CLOCK: &str = "cpu MHz";
pub(crate) const MEM_TOTAL: &str = "MemTotal:";
pub(crate) const MEM_FREE: &str = "MemAvailable:";
pub(crate) const SWAP_TOTAL: &str = "SwapTotal:";
pub(crate) const SWAP_FREE: &str = "SwapFree:";

pub(crate) fn ip(iface: &str) -> Result<serde_json::Value, Error> {
    let mut _ip = Command::new("ip");
    let mut cmd = if iface == "" {
        _ip.arg("-j").arg("address").arg("show")
    } else {
        _ip.arg("-j").arg("address").arg("show").arg(&iface)
    };
    Ok(serde_json::from_str::<serde_json::Value>(&run(&mut cmd)?)
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

fn _mem_extract(out: &str, line: &str) -> Result<usize, Error> {
    Ok(out
        .split('\n')
        .filter(|l| l.starts_with(line))
        .collect::<String>()
        .split_ascii_whitespace()
        .skip(1)
        .take(1)
        .collect::<String>()
        .parse::<usize>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?
        * 1024 as usize)
}

pub(crate) fn mem_extract(line: &str) -> Result<usize, Error> {
    _mem_extract(&SysPath::ProcMemInfo.read()?, &line)
}

fn _cpuinfo_extract<T: FromStr>(out: &str, line: &str) -> Result<T, Error>
where
    <T as FromStr>::Err: Display,
{
    Ok(out
        .split('\n')
        .filter(|l| l.starts_with(line))
        .take(1)
        .collect::<String>()
        .split(':')
        .skip(1)
        .take(1)
        .collect::<String>()
        .trim()
        .parse::<T>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn cpuinfo_extract<T: FromStr>(line: &str) -> Result<T, Error>
where
    <T as FromStr>::Err: Display,
{
    _cpuinfo_extract(&SysPath::ProcCpuInfo.read()?, &line)
}

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
    unsafe {
        let path = CString::new(path).map_err(|e| Error::InvalidInputError(path.to_string(), e.to_string()))?;
        let mode = CStr::from_bytes_with_nul_unchecked(b"r\0");
        let f = fopen(path.as_ptr(), mode.as_ptr());
        if f.is_null() {
            return Err(Error::FileReadError(
                path.to_string_lossy().to_string(),
                "failed to get file descriptor from `fopen`".to_string(),
            ));
        }
        let mut size: usize = 0;
        ioctl(fileno(f), BLKBSZGET, &mut size);

        Ok(size as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_meminfo() {
        assert_eq!(_mem_extract(MEMINFO, MEM_TOTAL), Ok(16712671232));
        assert_eq!(_mem_extract(MEMINFO, MEM_FREE), Ok(14993084416));
        assert_eq!(_mem_extract(MEMINFO, SWAP_TOTAL), Ok(0));
        assert_eq!(_mem_extract(MEMINFO, SWAP_FREE), Ok(0));
    }
    #[test]
    fn extracts_cpuinfo() {
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, CPU_CORES), Ok(6));
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, SIBLINGS), Ok(12));
        assert_eq!(_cpuinfo_extract::<f32>(CPUINFO, CPU_CLOCK), Ok(2053.971));
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
