//! # rsys
//! Crate for aquiring information about host machine and operating system
//! in a os-agnostic fashion.  
//!  
//! The common api is available through Rsys struct which compiles conditionally with
//! required methods. The error and result type is available at the root of this crate for convienience
//! while all the methods exposed by Rsys struct are also available in each os module.  
//!
//! Main goals are clear and easy api with as much of the api being os-agnostic.
//!   
//! ## Example usage:
//! ```
//! use rsys::{Result, Rsys};
//! fn main() -> Result<()> {
//!     // You can either use api through Rsys object
//!     // for os-agnostic experience
//!     let rsys = Rsys::new();
//!     println!("HOSTNAME - {}", rsys.hostname()?);
//!     let iface = rsys.default_iface()?;
//!     println!("CPU - {}", rsys.cpu()?);
//!     println!("ARCH - {}", rsys.arch()?);
//!     println!("MEMORY TOTAL - {}b", rsys.memory_total()?);
//!     println!("UPTIME - {}s", rsys.uptime()?);
//!     println!("SWAP TOTAL - {}b", rsys.swap_total()?);
//!     println!("CPU CORES - {}", rsys.cpu_cores()?);
//!     println!("CPU CLOCK - {}MHz", rsys.cpu_clock()?);
//!     
//!     // Or use functions in each module
//!     #[cfg(target_os = "linux")]
//!     {
//!         println!("KERNEL VERSION - {}", rsys::linux::kernel_version()?);
//!         println!("HOSTNAME - {}", rsys::linux::hostname()?);
//!
//!         // Os-specific functions are also available as methods
//!         println!("MEMORY - {:#?}", rsys.memory()?);
//!         println!("KERNEL_VERSION - {:#?}", rsys.kernel_version()?);
//!     }
//!     Ok(())
//! }
//! ```

#[cfg(target_os = "windows")]
extern crate winapi;

mod api;
mod error;
mod os;
pub(crate) mod util;
pub use api::Rsys;
pub use error::{RsysError as Error, RsysResult as Result};

#[cfg(target_os = "linux")]
pub use os::linux;
#[cfg(target_os = "macos")]
pub use os::macos;
#[cfg(target_os = "windows")]
pub use os::windows;
