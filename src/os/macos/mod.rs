//! MacOS specific api
mod cpu;
mod mem;
mod misc;
mod os_impl_ext;
mod system;

pub use crate::os::unix::arch;
pub use cpu::{cpu, cpu_clock, cpu_cores, logical_cores};
pub use mem::{memory_free, memory_total, swap_free, swap_total};
pub use misc::{domain_name, hostname, model, uptime};
pub use os_impl_ext::OsImplExt;

use crate::os::OsImpl;
use crate::Result;

#[derive(Default)]
pub(crate) struct MacOS {}
impl MacOS {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OsImpl for MacOS {
    fn hostname(&self) -> Result<String> {
        hostname()
    }

    fn domain_name(&self) -> Result<String> {
        domain_name()
    }

    fn uptime(&self) -> Result<u64> {
        uptime()
    }

    fn arch(&self) -> Result<String> {
        arch()
    }

    fn cpu(&self) -> Result<String> {
        cpu()
    }

    fn cpu_clock(&self) -> Result<f32> {
        cpu_clock()
    }

    fn cpu_cores(&self) -> Result<u16> {
        cpu_cores()
    }

    fn logical_cores(&self) -> Result<u16> {
        logical_cores()
    }

    fn memory_total(&self) -> Result<usize> {
        memory_total()
    }

    fn memory_free(&self) -> Result<usize> {
        memory_free()
    }

    fn swap_total(&self) -> Result<usize> {
        swap_total()
    }

    fn swap_free(&self) -> Result<usize> {
        swap_free()
    }
}
