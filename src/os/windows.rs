#![cfg(target_os = "windows")]
use super::Error;
use std::mem;
use winapi::{
    ctypes::c_char,
    um::{
        errhandlingapi::GetLastError,
        lmwksta::{NetWkstaGetInfo, WKSTA_INFO_100},
        profileapi::QueryPerformanceFrequency,
        shared::ntdef::{LARGE_INTEGER, NULL},
        sysinfoapi::{
            GetPhysicallyInstalledSystemMemory, GetSystemInfo, GetTickCount64, GlobalMemoryStatusEx, SYSTEM_INFO_u,
            MEMORYSTATUSEX, SYSTEM_INFO,
        },
        winbase::{
            FormatMessageA, GetComputerNameA, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
            FORMAT_MESSAGE_IGNORE_INSERTS,
        },
        winnt::{LANG_NEUTRAL, MAKELANGID, SUBLANG_DEFAULT},
    },
};

fn last_error() -> u32 {
    unsafe { GetLastError() }
}

fn utf8_buf_to_string(buf: &[i8]) -> Result<String, Error> {
    Ok(std::str::from_utf8(&buf.iter().map(|b| *b as u8).collect::<Vec<u8>>())
        .map_err(|e| Error::WinApiError(format!("got invalid utf8 string - `{}`", e)))?
        .to_string())
}

fn last_error_msg() -> Result<String, Error> {
    let mut out_buf: Vec<c_char> = Vec::new();
    let mut out_size: u32;

    unsafe {
        let last_id = last_error();
        FormatMessageA(
            FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            NULL,
            last_id,
            MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT),
            out_buf.as_mut_ptr(),
            out_size as *mut u32,
            NULL as *mut c_char,
        )
    }

    utf8_buf_to_string(&out_buf)
}

fn system_info() -> SYSTEM_INFO {
    unsafe {
        let mut info: SYSTEM_INFO = mem::zeroed();
        GetSystemInfo(info.as_mut_ptr());
        info
    }
}

fn memory_status() -> Result<MEMORYSTATUSEX, Error> {
    unsafe {
        let mut info: MEMORYSTATUSEX = mem::zeroed();
        let is_success = GlobalMemoryStatusEx(info.as_mut_ptr()) as bool;
        if !is_success {
            return Err(Error::WinApiError(last_error_msg()?));
        }
        Ok(info)
    }
}

fn net_wksta() -> WKSTA_INFO_100 {
    unsafe {
        let mut info: WKSTA_INFO_100 = mem::zeroed();
        // Check for error here
        NetWkstaGetInfo(NULL as *mut u16, 100, info.as_mut_ptr());
        info
    }
}

pub(crate) fn _hostname() -> Result<String, Error> {
    let mut out_buf: Vec<i8> = Vec::new();
    let mut out_size: u32;
    unsafe {
        let is_success = GetComputerNameA(out_buf.as_mut_ptr(), out_size as *mut u32) as bool;
        if !is_success {
            return Err(Error::WinApiError(last_error_msg()?));
        }
    }
    utf8_buf_to_string(&out_buf)
}

pub(crate) fn _default_iface() -> Result<String, Error> {
    todo!()
}

pub(crate) fn _domainname() -> Result<String, Error> {
    Ok(net_wksta().wki100_langroup)
}

pub(crate) fn _ipv4(iface: &str) -> Result<String, Error> {
    todo!()
}

pub(crate) fn _mac(iface: &str) -> Result<String, Error> {
    todo!()
}

pub(crate) fn _interfaces() -> Result<Vec<String>, Error> {
    todo!()
}

pub(crate) fn _ipv6(_iface: &str) -> Result<String, Error> {
    todo!()
}

pub(crate) fn _cpu() -> Result<String, Error> {
    todo!()
}

pub(crate) fn _cpu_cores() -> Result<u16, Error> {
    system_info().dwNumberOfProcessors
}

pub(crate) fn _cpu_clock() -> Result<f32, Error> {
    unsafe {
        let mut freq: LARGE_INTEGER = mem::zeroed();
        let is_success = QueryPerformanceFrequency(freq.as_mut_ptr()) as bool;
        if !is_success {
            return Err(Error::WinApiError(last_error_msg()?));
        }
        Ok(freq.QuadPart() as f32)
    }
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

pub(crate) fn _memory() -> Result<usize, Error> {
    Ok(memory_status()?.ullTotalPhys as usize)
}

pub(crate) fn _swap() -> Result<usize, Error> {
    Ok(memory_status()?.ullTotalVirtual as usize)
}

pub(crate) fn _uptime() -> Result<u64, Error> {
    unsafe { Ok((GetTickCount64() as u64) * 1000) }
}

pub(crate) fn _pagesize() -> u32 {
    system_info().dwPageSize
}
