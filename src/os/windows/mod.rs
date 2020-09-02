mod public;

use super::{Error, OsImpl};
use std::ffi::OsString;
use std::mem;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStringExt;
use std::ptr::{null, null_mut};
#[cfg(target_os = "windows")]
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

pub(crate) use public::*;

//################################################################################
// INTERNAL

const BUF_SIZE: usize = 4096;
const NUL: char = '\0';
const CARIAGE: char = '\r';
const NL: char = '\n';

//https://github.com/retep998/winapi-rs/issues/780
const MAX_COMPUTERNAME_LENGTH: u32 = 31;

#[cfg(target_os = "windows")]
fn last_error() -> u32 {
    unsafe { GetLastError() }
}

#[cfg(target_os = "windows")]
fn utf16_buf_to_string(buf: &[u16]) -> Result<String, Error> {
    Ok(OsString::from_wide(&buf)
        .to_string_lossy()
        .to_string()
        .trim_end_matches(NUL)
        .trim_end_matches(NL)
        .trim_end_matches(CARIAGE)
        .to_string())
}

#[cfg(target_os = "windows")]
fn last_error_msg() -> Result<(u32, String), Error> {
    let mut out_buf: Vec<u16> = vec![0; BUF_SIZE];
    let mut last_id = 0;
    unsafe {
        last_id = last_error();

        FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            null(),
            last_id,
            MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as u32,
            out_buf.as_mut_ptr(),
            BUF_SIZE as u32,
            null_mut(),
        );
    }

    utf16_buf_to_string(&out_buf).map(|s| (last_id, s))
}

#[cfg(target_os = "windows")]
fn system_info() -> SYSTEM_INFO {
    unsafe {
        let mut info: SYSTEM_INFO = mem::zeroed();
        GetSystemInfo(&mut info);
        info
    }
}

#[cfg(target_os = "windows")]
fn memory_status() -> Result<MEMORYSTATUSEX, Error> {
    unsafe {
        let mut info = mem::zeroed::<MEMORYSTATUSEX>();
        info.dwLength = mem::size_of::<MEMORYSTATUSEX>() as u32;
        let is_success = GlobalMemoryStatusEx(&mut info) != 0;
        if !is_success {
            let (id, msg) = last_error_msg()?;
            return Err(Error::WinApiError(id, msg));
        }
        Ok(info)
    }
}

// #[cfg(target_os = "windows")]
// fn net_wksta() -> WKSTA_INFO_100 {
//     unsafe {
//         let mut info: WKSTA_INFO_100 = mem::zeroed();
//         // Check for error here
//         NetWkstaGetInfo(NULL as *mut u16, 100, &mut info);
//         info
//     }
// }

#[cfg(target_os = "windows")]
fn logical_processor_information() -> Result<Vec<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>, Error> {
    unsafe {
        let x = mem::zeroed::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>();
        let mut info = vec![x; BUF_SIZE];
        let mut ret_length: u32 = BUF_SIZE as u32;
        let is_success = GetLogicalProcessorInformation(info.as_mut_ptr(), &mut ret_length) != 0;
        if !is_success {
            let (id, msg) = last_error_msg()?;
            return Err(Error::WinApiError(id, msg));
        }
        Ok(info)
    }
}

#[cfg(target_os = "windows")]
fn is_cpu_hyperthreaded() -> Result<bool, Error> {
    unsafe { Ok(logical_processor_information()?[0].u.ProcessorCore().Flags == 1) }
}

#[cfg(target_os = "windows")]
#[allow(dead_code)]
fn pagesize() -> u32 {
    system_info().dwPageSize
}

#[cfg(target_os = "windows")]
fn reg_val<T>(key: HKEY, subkey: &str, val: &str) -> Result<T, Error> {
    unsafe {
        let mut hkey = mem::zeroed::<HKEY>();
        let mut subkey = subkey.encode_utf16().collect::<Vec<u16>>();
        println!("main key = `{:?}`", key);
        println!("sub key = `{}`", subkey);
        let mut is_success = RegOpenKeyExW(key, subkey.as_ptr(), 0, KEY_READ, &mut hkey) as u32 != ERROR_SUCCESS;
        if !is_success {
            let (id, msg) = last_error_msg()?;
            return Err(Error::WinApiError(id, msg));
        }
        let mut buf_size = mem::size_of::<T>();
        let mut dbuf = vec![0u8; buf_size];
        let mut val = val.encode_utf16().collect::<Vec<u16>>();
        let mut tbuf = vec![0u32; buf_size];
        let mut null = 0;
        is_success = RegQueryValueExW(
            hkey,
            val.as_ptr(),
            null as *mut u32,
            tbuf.as_mut_ptr(),
            dbuf.as_mut_ptr(),
            buf_size as *mut u32,
        ) as u32
            != ERROR_SUCCESS;
        if !is_success {
            let (id, msg) = last_error_msg()?;
            return Err(Error::WinApiError(id, msg));
        }
        println!("dbuf = `{:?}`", &dbuf);
        println!("tbuf = `{:?}`", &tbuf);
        Ok(mem::transmute_copy::<Vec<u8>, T>(&dbuf))
    }
}
