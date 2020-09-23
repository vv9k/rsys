//! Windows specific api
mod internal;
mod os_impl_ext;
mod public;

use super::OsImpl;
use crate::Result;
use std::{
    ffi::OsString,
    mem,
    os::windows::ffi::OsStringExt,
    ptr::{null, null_mut},
};
use winapi::{
    shared::{minwindef::HKEY, winerror::ERROR_SUCCESS},
    um::{
        errhandlingapi::GetLastError,
        //lmwksta::{NetWkstaGetInfo, WKSTA_INFO_100},
        sysinfoapi::{
            GetLogicalProcessorInformation, GetSystemInfo, GetTickCount64, GlobalMemoryStatusEx, MEMORYSTATUSEX,
            SYSTEM_INFO,
        },
        winbase::{FormatMessageW, GetComputerNameW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS},
        winnt::{
            KEY_READ,
            LANG_NEUTRAL,
            MAKELANGID,
            SUBLANG_DEFAULT,
            SYSTEM_LOGICAL_PROCESSOR_INFORMATION,
            //SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION,
        },
        winreg::{RegOpenKeyExW, RegQueryValueExW, HKEY_LOCAL_MACHINE},
    },
};

pub(crate) use internal::*;
pub use os_impl_ext::OsImplExt;
pub use public::*;

const BUF_SIZE: usize = 4096;
const NUL: char = '\0';
const CARIAGE: char = '\r';
const NL: char = '\n';

//https://github.com/retep998/winapi-rs/issues/780
const MAX_COMPUTERNAME_LENGTH: u32 = 31;

#[derive(Default, OsImpl)]
pub(crate) struct Windows {}
impl OsImplExt for Windows {}
