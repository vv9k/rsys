use super::*;
use crate::Result;

pub fn hostname() -> Result<String> {
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

pub fn uptime() -> Result<u64> {
    unsafe { Ok((GetTickCount64() as u64) * 1000) }
}

pub fn arch() -> Result<String> {
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

pub fn cpu() -> Result<String> {
    Ok("".to_string())
}

// # TODO
// Figure out why the registry is returning an empty buffer (probably not finding the right hkey?)
pub fn cpu_clock() -> Result<f32> {
    reg_val::<u32>(
        HKEY_LOCAL_MACHINE,
        "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0",
        "~MHz",
    )
    .map(|v| v as f32)
}

pub fn cpu_cores() -> Result<u16> {
    if is_cpu_hyperthreaded()? {
        Ok(logical_cores()? / 2)
    } else {
        Ok(logical_cores()?)
    }
}

pub fn logical_cores() -> Result<u16> {
    Ok(system_info().dwNumberOfProcessors as u16)
}

pub fn memory_total() -> Result<usize> {
    Ok(memory_status()?.ullTotalPhys as usize)
}

pub fn memory_free() -> Result<usize> {
    Ok(memory_status()?.ullAvailPhys as usize)
}

pub fn swap_total() -> Result<usize> {
    Ok(memory_status()?.ullTotalVirtual as usize)
}

pub fn swap_free() -> Result<usize> {
    Ok(memory_status()?.ullAvailVirtual as usize)
}

pub fn default_iface() -> Result<String> {
    Ok("".to_string())
}

pub fn ipv4(iface: &str) -> Result<String> {
    Ok("".to_string())
}

pub fn ipv6(_iface: &str) -> Result<String> {
    Ok("".to_string())
}

pub fn mac(iface: &str) -> Result<String> {
    Ok("".to_string())
}

pub fn interfaces() -> Result<Vec<String>> {
    Ok(vec![])
}

pub fn domainname() -> Result<String> {
    // Ok(net_wksta().wki100_langroup)
    Ok("".to_string())
}

//################################################################################
// UNIQUE

/// Returns a number between 0 and 100 that specifies the approximate percentage of physical memory that is in use
/// (0 indicates no memory use and 100 indicates full memory use).
pub fn memory_load() -> Result<u32> {
    Ok(memory_status()?.dwMemoryLoad as u32)
}
