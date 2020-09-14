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
    net::{IfaceDev, Ifaces},
    ps::{Process, ProcessState, Processes},
    storage::{BlockStorage, BlockStorageStat, DeviceMapper, Partition, Partitions, ScsiCdrom},
    MountPoint, MountPoints,
};

pub(crate) use internal::*;
pub(crate) use sysproc::SysPath;

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
