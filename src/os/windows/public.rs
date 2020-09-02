use super::*;

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
    reg_val::<u32>(
        HKEY_LOCAL_MACHINE,
        "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0",
        "~MHz",
    )
    .map(|v| v as f32)
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
