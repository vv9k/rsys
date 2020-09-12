//! Linux specific api
#![cfg(target_os = "linux")]

mod internal;
#[cfg(test)]
pub(crate) mod mocks;
mod public;
mod sysproc;
mod types;

use super::{run, Error, OsImpl};

pub use public::*;
pub use types::{
    BlockStorage, BlockStorageStat, IfaceDev, Ifaces, MountPoint, MountPoints, Process, ProcessState, Processes,
};

pub(crate) use internal::*;
pub(crate) use sysproc::SysPath;

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
