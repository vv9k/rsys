use super::Error;
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

//################################################################################
// INTERNAL

const BUF_SIZE: usize = 4096;
const NUL: char = '\0';
const CARIAGE: char = '\r';
const NL: char = '\n';

//https://github.com/retep998/winapi-rs/issues/780
const MAX_COMPUTERNAME_LENGTH: u32 = 31;

fn last_error() -> u32 {
    unsafe { GetLastError() }
}

fn utf16_buf_to_string(buf: &[u16]) -> Result<String, Error> {
    Ok(OsString::from_wide(&buf)
        .to_string_lossy()
        .to_string()
        .trim_end_matches(NUL)
        .trim_end_matches(NL)
        .trim_end_matches(CARIAGE)
        .to_string())
}

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

fn system_info() -> SYSTEM_INFO {
    unsafe {
        let mut info: SYSTEM_INFO = mem::zeroed();
        GetSystemInfo(&mut info);
        info
    }
}

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

// fn net_wksta() -> WKSTA_INFO_100 {
//     unsafe {
//         let mut info: WKSTA_INFO_100 = mem::zeroed();
//         // Check for error here
//         NetWkstaGetInfo(NULL as *mut u16, 100, &mut info);
//         info
//     }
// }

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

fn is_cpu_hyperthreaded() -> Result<bool, Error> {
    unsafe { Ok(logical_processor_information()?[0].u.ProcessorCore().Flags == 1) }
}

#[allow(dead_code)]
fn pagesize() -> u32 {
    system_info().dwPageSize
}

fn reg_lm_val<T>(subkey: &str, val: &str) -> Result<T, Error> {
    unsafe {
        let mut hkey = mem::zeroed::<HKEY>();
        let mut subkey = subkey.encode_utf16().collect::<Vec<u16>>();
        println!("hkey = `{:?}`", hkey);
        let mut is_success =
            RegOpenKeyExW(HKEY_LOCAL_MACHINE, subkey.as_ptr(), 0, KEY_READ, &mut hkey) as u32 != ERROR_SUCCESS;
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

//################################################################################
// PUBLIC

pub(crate) fn _hostname() -> Result<String, Error> {
    let mut out_buf: Vec<u16> = vec![0; BUF_SIZE];
    let mut out_size: u32 = MAX_COMPUTERNAME_LENGTH;
    unsafe {
        let ret = GetComputerNameW(out_buf.as_mut_ptr(), &mut out_size);
        if ret == 0 {
            let (id, msg) = last_error_msg()?;
            return Err(Error::WinApiError(id, msg));
        }
    }
    utf16_buf_to_string(&out_buf)
}

pub(crate) fn _uptime() -> Result<u64, Error> {
    unsafe { Ok((GetTickCount64() as u64) * 1000) }
}

pub(crate) fn _arch() -> Result<String, Error> {
    unsafe {
        let arch = match system_info().u.s().wProcessorArchitecture {
            9 => "x64",
            5 => "ARM",
            12 => "ARM64",
            6 => "Intel Itanium-based",
            0 => "x86",
            _ => "",
        };
        Ok(arch.to_string())
    }
}

pub(crate) fn _cpu() -> Result<String, Error> {
    todo!()
}

// # TODO
// Figure out why the registry is returning an empty buffer (probably not finding the right hkey?)
pub(crate) fn _cpu_clock() -> Result<f32, Error> {
    reg_lm_val::<u32>("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0", "~MHz").map(|v| v as f32)
}

pub(crate) fn _cpu_cores() -> Result<u16, Error> {
    if is_cpu_hyperthreaded()? {
        Ok(_logical_cores()? / 2)
    } else {
        Ok(_logical_cores()?)
    }
}

pub(crate) fn _logical_cores() -> Result<u16, Error> {
    Ok(system_info().dwNumberOfProcessors as u16)
}

pub(crate) fn _memory() -> Result<usize, Error> {
    Ok(memory_status()?.ullTotalPhys as usize)
}

pub(crate) fn _swap() -> Result<usize, Error> {
    Ok(memory_status()?.ullTotalVirtual as usize)
}

pub(crate) fn _default_iface() -> Result<String, Error> {
    todo!()
}

pub(crate) fn _ipv4(iface: &str) -> Result<String, Error> {
    todo!()
}

pub(crate) fn _ipv6(_iface: &str) -> Result<String, Error> {
    todo!()
}

pub(crate) fn _mac(iface: &str) -> Result<String, Error> {
    todo!()
}

pub(crate) fn _interfaces() -> Result<Vec<String>, Error> {
    todo!()
}

pub(crate) fn _domainname() -> Result<String, Error> {
    // Ok(net_wksta().wki100_langroup)
    todo!()
}
