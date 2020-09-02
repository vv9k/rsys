use super::*;

pub(crate) fn last_error() -> u32 {
    unsafe { GetLastError() }
}

pub(crate) fn utf16_buf_to_string(buf: &[u16]) -> Result<String, Error> {
    Ok(OsString::from_wide(&buf)
        .to_string_lossy()
        .to_string()
        .trim_end_matches(NUL)
        .trim_end_matches(NL)
        .trim_end_matches(CARIAGE)
        .to_string())
}

pub(crate) fn last_error_msg() -> Result<(u32, String), Error> {
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

pub(crate) fn system_info() -> SYSTEM_INFO {
    unsafe {
        let mut info: SYSTEM_INFO = mem::zeroed();
        GetSystemInfo(&mut info);
        info
    }
}

pub(crate) fn memory_status() -> Result<MEMORYSTATUSEX, Error> {
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
// pub(crate) fn net_wksta() -> WKSTA_INFO_100 {
//     unsafe {
//         let mut info: WKSTA_INFO_100 = mem::zeroed();
//         // Check for error here
//         NetWkstaGetInfo(NULL as *mut u16, 100, &mut info);
//         info
//     }
// }

pub(crate) fn logical_processor_information() -> Result<Vec<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>, Error> {
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

pub(crate) fn is_cpu_hyperthreaded() -> Result<bool, Error> {
    unsafe { Ok(logical_processor_information()?[0].u.ProcessorCore().Flags == 1) }
}

#[allow(dead_code)]
pub(crate) fn pagesize() -> u32 {
    system_info().dwPageSize
}

pub(crate) fn reg_val<T>(key: HKEY, subkey: &str, val: &str) -> Result<T, Error> {
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
