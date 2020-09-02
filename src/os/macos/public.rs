use super::*;

pub(crate) fn _hostname() -> Result<String, Error> {
    sysctl(SYSCTL_HOSTNAME)
}

pub(crate) fn _uptime() -> Result<u64, Error> {
    let boot = sysctl(SYSCTL_BOOTTIME)?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::TimeError(e.to_string()))?
        .as_secs();
    let boottime = boot[SYSCTL_BOOTTIME_LEN..SYSCTL_BOOTTIME_LEN + format!("{}", now).len()]
        .parse::<u64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?;
    Ok(now - boottime)
}

pub(crate) fn _arch() -> Result<String, Error> {
    run(Command::new("uname").arg("-m"))
}

pub(crate) fn _cpu() -> Result<String, Error> {
    sysctl(SYSCTL_CPU)
}

pub(crate) fn _cpu_clock() -> Result<f32, Error> {
    Ok(sysctl(CPU_FREQUENCY)?
        .parse::<u64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))
        .map(|v| (v / 1_000_000) as f32)?)
}

pub(crate) fn _cpu_cores() -> Result<u16, Error> {
    Ok(sysctl(CPU_CORES)?
        .parse::<u16>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn _logical_cores() -> Result<u16, Error> {
    Ok(sysctl(LOGICAL_CORES)?
        .parse::<u16>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn _memory() -> Result<usize, Error> {
    Ok(sysctl(SYSCTL_MEMSIZE)?
        .parse::<usize>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn _swap() -> Result<usize, Error> {
    let (mut active, mut inactive) = (0, 0);
    let (mut was_active, mut was_inactive) = (false, false);
    let pagesize = vm_pagesize()?;
    let mut cmd = Command::new("vm_stat");
    for line in run(&mut cmd)?.split('\n') {
        if line.starts_with(PAGES_ACTIVE) {
            active = line
                .split_ascii_whitespace()
                .last()
                .ok_or_else(|| {
                    Error::InvalidInputError(format!("line containing active pages was invalid `{}`", line))
                })?
                .trim_end_matches('.')
                .parse::<u64>()
                .map_err(|e| Error::CommandParseError(e.to_string()))?;
            was_active = true;
        }
        if line.starts_with(PAGES_INACTIVE) {
            inactive = line
                .split_ascii_whitespace()
                .last()
                .ok_or_else(|| {
                    Error::InvalidInputError(format!("line containing inactive pages was invalid `{}`", line))
                })?
                .trim_end_matches('.')
                .parse::<u64>()
                .map_err(|e| Error::CommandParseError(e.to_string()))?;
            was_inactive = true;
        }
        if was_active && was_inactive {
            break;
        }
    }

    Ok(((active + inactive) * pagesize as u64) as usize)
}

pub(crate) fn default_iface() -> Result<String, Error> {
    let out = run(Command::new("route").arg("get").arg("default"))?;
    if let Some(ifc_line) = out.split('\n').find(|l| l.trim().starts_with(INTERFACE)) {
        return Ok(ifc_line.trim()[INTERFACE_LEN..].trim_end_matches('\n').to_string());
    }

    Ok("".to_string())
}

pub(crate) fn _ipv4(iface: &str) -> Result<String, Error> {
    run(Command::new("ipconfig").arg("getifaddr").arg(iface))
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
    todo!()
}
