//! # rsys
//!
//! Crate for aquiring information about host machine and operating system
//! in a os-agnostic fashion.  
//!  
//! The common api is available through Rsys struct which compiles conditionally with
//! required methods. The error type is available at the root of this crate for convienience
//! while os-specific functions are hidden in submodules corresponding to each os.  
//!   
//! ## Example usage:
//! ```rust,ignore
//! use rsys::{Error, Rsys};
//!
//! fn main() -> Result<(), Error> {
//!     // You can either use api through Rsys object
//!     // for os-agnostic experience
//!     let rsys = Rsys::new();
//!     println!("HOSTNAME - {}", rsys.hostname()?);
//!     let iface = rsys.default_iface()?;
//!     println!("CPU - {}", rsys.cpu()?);
//!     println!("ARCH - {}", rsys.arch()?);
//!     println!("MEMORY - {} b", rsys.memory()?);
//!     println!("UPTIME - {} s", rsys.uptime()?);
//!     println!("SWAP - {}b", rsys.swap()?);
//!     println!("CPU CORES - {}", rsys.cpu_cores()?);
//!     println!("CPU CLOCK - {} MHz", rsys.cpu_clock()?);
//!     println!("IPv4 - {}", rsys.ipv4(&iface)?);
//!     println!("MAC - {}", rsys.mac(&iface)?);
//!     println!("INTERFACES - {:#?}", rsys.interfaces()?);
//!     
//!     // Or use functions in each module
//!     if cfg!(target_os = "linux") {
//!         println!("KERNEL VERSION - {}", rsys::linux::kernel_version()?);
//!         println!("HOSTNAME - {}", rsys::linux::hostname()?);
//!     }
//!
//!     Ok(())
//! }
//! ```
#[cfg(target_os = "windows")]
extern crate winapi;

#[macro_use]
extern crate rsys_macro;

mod api;
mod error;
mod os;
pub(crate) mod util;
pub use api::Rsys;
pub use error::RsysError as Error;

#[cfg(target_os = "linux")]
pub use os::linux;
#[cfg(target_os = "macos")]
pub use os::macos;
#[cfg(target_os = "windows")]
pub use os::windows;
