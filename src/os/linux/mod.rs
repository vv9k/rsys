//! Linux specific api
#![cfg(target_os = "linux")]

mod internal;
mod procfs;
mod public;

use super::{run, Error, OsImpl};

pub use public::*;

pub(crate) use internal::*;
pub(crate) use procfs::ProcPath;

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
