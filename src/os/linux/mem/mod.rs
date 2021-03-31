use super::sysinfo;
use crate::Result;

/// Returns total memory.
pub fn memory_total() -> Result<usize> {
    sysinfo().map(|s| s.ram_total() as usize)
}

/// Returns free memory
pub fn memory_free() -> Result<usize> {
    sysinfo().map(|s| s.ram_unused() as usize)
}

/// Returns total swap space.
pub fn swap_total() -> Result<usize> {
    sysinfo().map(|s| s.swap_total() as usize)
}

/// Returns free swap space.
pub fn swap_free() -> Result<usize> {
    sysinfo().map(|s| s.swap_free() as usize)
}
