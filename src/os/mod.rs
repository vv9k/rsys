#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub(crate) use linux::OsImplExt;
#[cfg(target_os = "macos")]
pub(crate) use macos::OsImplExt;
#[cfg(target_os = "windows")]
pub(crate) use windows::OsImplExt;

pub(crate) mod os_impl;
pub(crate) use os_impl::OsImpl;

use crate::{Error, Result};
use std::{process::Command, str};

// Internal function for mapping errors on command execution
fn run(cmd: &mut Command) -> Result<String> {
    match cmd.output() {
        Ok(out) => match str::from_utf8(&out.stdout) {
            Ok(out) => Ok(out.trim_end_matches('\n').to_string()),
            Err(e) => Err(Error::CommandParseError(e.to_string())),
        },
        Err(e) => Err(Error::CommandRunError(e.to_string())),
    }
}
