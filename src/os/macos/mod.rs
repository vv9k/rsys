//! MacOS specific api
mod cpu;
mod mem;
mod misc;
mod net;
mod os_impl_ext;
mod system;

pub use crate::os::unix::arch;
pub use cpu::{cpu, cpu_clock, cpu_cores, logical_cores};
pub use mem::{memory_free, memory_total, swap_free, swap_total};
pub use misc::{domainname, hostname, model, uptime};
pub use net::{default_iface, interfaces, ipv4, ipv6, mac};
pub use os_impl_ext::OsImplExt;

use crate::os::OsImpl;
use crate::Result;

#[derive(Default, OsImpl)]
pub(crate) struct Macos {}
impl OsImplExt for Macos {}
