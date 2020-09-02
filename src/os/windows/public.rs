use super::*;

#[derive(Default)]
pub(crate) struct Windows {}
impl Windows {
    pub fn new() -> Self {
        Self::default()
    }
}
#[cfg(target_os = "windows")]
impl OsImpl for Windows {
    fn hostname(&self) -> Result<String, Error> {
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

    fn uptime(&self) -> Result<u64, Error> {
        unsafe { Ok((GetTickCount64() as u64) * 1000) }
    }

    fn arch(&self) -> Result<String, Error> {
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

    fn cpu(&self) -> Result<String, Error> {
        todo!()
    }

    // # TODO
    // Figure out why the registry is returning an empty buffer (probably not finding the right hkey?)
    fn cpu_clock(&self) -> Result<f32, Error> {
        reg_val::<u32>(
            HKEY_LOCAL_MACHINE,
            "HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0",
            "~MHz",
        )
        .map(|v| v as f32)
    }

    fn cpu_cores(&self) -> Result<u16, Error> {
        if is_cpu_hyperthreaded()? {
            Ok(_logical_cores()? / 2)
        } else {
            Ok(_logical_cores()?)
        }
    }

    fn logical_cores(&self) -> Result<u16, Error> {
        Ok(system_info().dwNumberOfProcessors as u16)
    }

    fn memory(&self) -> Result<usize, Error> {
        Ok(memory_status()?.ullTotalPhys as usize)
    }

    fn swap(&self) -> Result<usize, Error> {
        Ok(memory_status()?.ullTotalVirtual as usize)
    }

    fn default_iface(&self) -> Result<String, Error> {
        todo!()
    }

    fn ipv4(&self, iface: &str) -> Result<String, Error> {
        todo!()
    }

    fn ipv6(&self, _iface: &str) -> Result<String, Error> {
        todo!()
    }

    fn mac(&self, iface: &str) -> Result<String, Error> {
        todo!()
    }

    fn interfaces(&self) -> Result<Vec<String>, Error> {
        todo!()
    }

    fn domainname(&self) -> Result<String, Error> {
        // Ok(net_wksta().wki100_langroup)
        todo!()
    }
}
