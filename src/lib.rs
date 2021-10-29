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
//!
//! fn main() -> Result<()> {
//!     // You can either use api through Rsys object
//!     // for os-agnostic experience
//!     let rsys = Rsys::new();
//!     println!("HOSTNAME - {}", rsys.hostname()?);
//!     println!("CPU - {}", rsys.cpu()?);
//!     println!("CPU CORES - {}", rsys.cpu_cores()?);
//!     println!("CPU CLOCK - {}MHz", rsys.cpu_clock()?);
//!     println!("UPTIME - {}s", rsys.uptime()?);
//!     println!("ARCH - {}", rsys.arch()?);
//!     println!("SWAP TOTAL - {}b", rsys.swap_total()?);
//!     println!("SWAP FREE - {}b", rsys.swap_free()?);
//!     println!("MEMORY TOTAL - {}b", rsys.memory_total()?);
//!     println!("MEMORY FREE - {}b", rsys.memory_free()?);
//!
//!     #[cfg(target_os = "linux")]
//!         {
//!             // Or use functions in each module
//!             println!("KERNEL VERSION - {}", rsys::linux::kernel_release()?);
//!
//!             // Os-specific functions are also available as methods
//!             println!("KERNEL_VERSION - {}", rsys.kernel_release()?);
//!             println!("{:#?}", rsys.pids()?);
//!             println!("MOUNTS - {:#?}", rsys::linux::mounts::mounts()?);
//!         }
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
