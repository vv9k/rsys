#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(unix)]
pub(crate) mod unix;
#[cfg(windows)]
pub mod windows;

#[cfg(target_os = "linux")]
pub(crate) use linux::OsImplExt;
#[cfg(target_os = "macos")]
pub(crate) use macos::OsImplExt;
#[cfg(target_os = "windows")]
pub(crate) use windows::OsImplExt;

pub(crate) mod os_impl;
pub(crate) use os_impl::OsImpl;
