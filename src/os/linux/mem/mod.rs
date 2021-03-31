use super::sysinfo;
use crate::Result;

/// Returns the total amount of installed RAM in Bytes.
pub fn memory_total() -> Result<usize> {
    sysinfo().map(|s| s.memory_total() as usize)
}

/// Returns the amount of completely unused RAM in Bytes.
///
/// "Unused" in this context means that the RAM in neither actively used by
/// programs, nor by the operating system as disk cache or buffer. It is
/// "wasted" RAM since it currently serves no purpose.
pub fn memory_free() -> Result<usize> {
    sysinfo().map(|s| s.memory_free() as usize)
}

/// Returns the amount of swap memory in Bytes.
pub fn swap_total() -> Result<usize> {
    sysinfo().map(|s| s.swap_total() as usize)
}

/// Returns the amount of unused swap memory in Bytes.
pub fn swap_free() -> Result<usize> {
    sysinfo().map(|s| s.swap_free() as usize)
}

/// Returns the total amount of shared RAM in Bytes.
pub fn memory_shared() -> Result<usize> {
    sysinfo().map(|s| s.memory_shared() as usize)
}

/// Returns the total amount of memory used by buffers in Bytes.
pub fn memory_buffered() -> Result<usize> {
    sysinfo().map(|s| s.memory_buffered() as usize)
}

/// Returns the total high memory size in Bytes.
pub fn memory_high_total() -> Result<usize> {
    sysinfo().map(|s| s.memory_high_total() as usize)
}

/// Returns the total amount of unused high memory size in Bytes.
pub fn memory_high_free() -> Result<usize> {
    sysinfo().map(|s| s.memory_high_free() as usize)
}
