//! MacOS specific api
mod cpu;
mod mem;
mod misc;
mod os_impl_ext;
mod system;

pub use crate::os::unix::arch;
pub use cpu::{cpu, cpu_clock, cpu_cores, logical_cores};
pub use mem::{memory_free, memory_total, swap_free, swap_total};
pub use misc::{domainname, hostname, model, uptime};
pub use os_impl_ext::OsImplExt;

use crate::os::OsImpl;
use crate::Result;

#[derive(Default, OsImpl)]
pub(crate) struct Macos {}
