//! Linux specific api
#![cfg(target_os = "linux")]

mod internal;
mod procfs;
mod public;
mod types;

use super::{run, Error, OsImpl};

pub use public::*;
pub use types::{MountPoint, MountPoints};

pub(crate) use internal::*;
pub(crate) use procfs::ProcPath;

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
