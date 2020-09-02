//! Windows specific api
mod internal;
mod public;

use super::{Error, OsImpl};
use std::ffi::OsString;
use std::mem;
use std::os::windows::ffi::OsStringExt;
use std::ptr::{null, null_mut};
use winapi::{
    shared::{
        minwindef::{HKEY, MAX_PATH},
        winerror::ERROR_SUCCESS,
    },
    um::{
        errhandlingapi::GetLastError,
        lmwksta::{NetWkstaGetInfo, WKSTA_INFO_100},
        sysinfoapi::{
            GetLogicalProcessorInformation, GetProcessorSystemCycleTime, GetSystemInfo, GetTickCount64,
            GlobalMemoryStatusEx, MEMORYSTATUSEX, SYSTEM_INFO,
        },
        winbase::{FormatMessageW, GetComputerNameW, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS},
        winnt::{
            KEY_READ, LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT, SYSTEM_LOGICAL_PROCESSOR_INFORMATION,
            SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION,
        },
        winreg::{RegOpenKeyExW, RegQueryValueExW, HKEY_LOCAL_MACHINE},
    },
};

pub(crate) use internal::*;
pub use public::*;

const BUF_SIZE: usize = 4096;
const NUL: char = '\0';
const CARIAGE: char = '\r';
const NL: char = '\n';

//https://github.com/retep998/winapi-rs/issues/780
const MAX_COMPUTERNAME_LENGTH: u32 = 31;

#[derive(Default, OsImpl)]
pub(crate) struct Windows {}
