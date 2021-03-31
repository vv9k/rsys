#[cfg(test)]
use super::mocks::MEMINFO;
use super::SysPath;
use crate::{Error, Result};
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

const MEM_TOTAL: &str = "MemTotal:";
const MEM_FREE: &str = "MemFree:";
const MEM_AVAILABLE: &str = "MemAvailable:";
const SWAP_TOTAL: &str = "SwapTotal:";
const SWAP_FREE: &str = "SwapFree:";
const MEM_BUFFERS: &str = "Buffers:";
const MEM_CACHED: &str = "Cached:";
const MEM_ACTIVE: &str = "Active:";
const MEM_INACTIVE: &str = "Inactive:";
const MEM_SHARED: &str = "Shmem:";

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Memory {
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub buffers: u64,
    pub cached: u64,
    pub active: u64,
    pub inactive: u64,
    pub shared: u64,
}
impl Memory {
    pub fn from_proc() -> Result<Memory> {
        Self::from_sys_path(SysPath::ProcMemInfo)
    }

    fn from_sys_path(path: SysPath) -> Result<Memory> {
        let mut mem = Memory::default();
        for line in path.read()?.lines() {
            let mut elems = line.split_ascii_whitespace();
            if let Some(p) = elems.next() {
                let param = match p {
                    MEM_TOTAL => &mut mem.total,
                    MEM_FREE => &mut mem.free,
                    MEM_AVAILABLE => &mut mem.available,
                    MEM_BUFFERS => &mut mem.buffers,
                    MEM_CACHED => &mut mem.cached,
                    MEM_ACTIVE => &mut mem.active,
                    MEM_INACTIVE => &mut mem.inactive,
                    MEM_SHARED => &mut mem.shared,
                    _ => continue,
                };
                if let Some(v) = elems.next() {
                    *param = v
                        .parse::<u64>()
                        .map(|v| v * 1024) // Each value is in KiloBytes
                        .map_err(|e| Error::InvalidInputError(v.to_string(), e.to_string()))?;
                }
            }
        }

        Ok(mem)
    }
}

fn _mem_extract(out: &str, line: &str) -> Result<usize> {
    Ok(out
        .lines()
        .filter(|l| l.starts_with(line))
        .collect::<String>()
        .split_ascii_whitespace()
        .skip(1)
        .take(1)
        .collect::<String>()
        .parse::<usize>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?
        * 1024_usize)
}

pub(crate) fn mem_extract(line: &str) -> Result<usize> {
    _mem_extract(&SysPath::ProcMemInfo.read()?, &line)
}

/// Returns total memory.
pub fn memory_total() -> Result<usize> {
    mem_extract(MEM_TOTAL)
}

/// Returns free memory
pub fn memory_free() -> Result<usize> {
    mem_extract(MEM_FREE)
}

/// Returns total swap space.
pub fn swap_total() -> Result<usize> {
    mem_extract(SWAP_TOTAL)
}

/// Returns free swap space.
pub fn swap_free() -> Result<usize> {
    mem_extract(SWAP_FREE)
}

/// Returns detailed information about memory
pub fn memory() -> Result<Memory> {
    Memory::from_proc()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io};
    #[test]
    fn extracts_meminfo() {
        assert_eq!(_mem_extract(MEMINFO, MEM_TOTAL), Ok(16712671232));
        assert_eq!(_mem_extract(MEMINFO, MEM_AVAILABLE), Ok(14993084416));
        assert_eq!(_mem_extract(MEMINFO, SWAP_TOTAL), Ok(0));
        assert_eq!(_mem_extract(MEMINFO, SWAP_FREE), Ok(0));
    }
    #[test]
    fn creates_memory_struct() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let p = dir.path().join("meminfo");
        fs::write(p.as_path(), MEMINFO)?;

        let mem = Memory {
            active: 1848623104,
            available: 14993084416,
            buffers: 130609152,
            cached: 2210324480,
            free: 12829442048,
            inactive: 1351041024,
            shared: 45924352,
            total: 16712671232,
        };

        assert_eq!(Ok(mem), Memory::from_sys_path(SysPath::Custom(p)));

        dir.close()
    }
}
