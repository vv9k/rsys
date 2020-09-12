//! Linux specific api
#![cfg(target_os = "linux")]

mod internal;
#[cfg(test)]
pub(crate) mod mocks;
mod procfs;
mod public;
mod types;

use super::{run, Error, OsImpl};

pub use public::*;
pub use types::{IfaceDev, Ifaces, MountPoint, MountPoints, Process, ProcessState, Processes};

pub(crate) use internal::*;
pub(crate) use procfs::ProcPath;

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
