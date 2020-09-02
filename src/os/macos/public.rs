use super::*;

#[derive(Default)]
pub(crate) struct Macos {}
impl Macos {
    pub fn new() -> Self {
        Self::default()
    }
}
impl OsImpl for Macos {
    fn hostname(&self) -> Result<String, Error> {
        sysctl(SYSCTL_HOSTNAME)
    }

    fn uptime(&self) -> Result<u64, Error> {
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

    fn arch(&self) -> Result<String, Error> {
        run(Command::new("uname").arg("-m"))
    }

    fn cpu(&self) -> Result<String, Error> {
        sysctl(SYSCTL_CPU)
    }

    fn cpu_clock(&self) -> Result<f32, Error> {
        Ok(sysctl(CPU_FREQUENCY)?
            .parse::<u64>()
            .map_err(|e| Error::CommandParseError(e.to_string()))
            .map(|v| (v / 1_000_000) as f32)?)
    }

    fn cpu_cores(&self) -> Result<u16, Error> {
        Ok(sysctl(CPU_CORES)?
            .parse::<u16>()
            .map_err(|e| Error::CommandParseError(e.to_string()))?)
    }

    fn logical_cores(&self) -> Result<u16, Error> {
        Ok(sysctl(LOGICAL_CORES)?
            .parse::<u16>()
            .map_err(|e| Error::CommandParseError(e.to_string()))?)
    }

    fn memory(&self) -> Result<usize, Error> {
        Ok(sysctl(SYSCTL_MEMSIZE)?
            .parse::<usize>()
            .map_err(|e| Error::CommandParseError(e.to_string()))?)
    }

    fn swap(&self) -> Result<usize, Error> {
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

    fn default_iface(&self) -> Result<String, Error> {
        let out = run(Command::new("route").arg("get").arg("default"))?;
        if let Some(ifc_line) = out.split('\n').find(|l| l.trim().starts_with(INTERFACE)) {
            return Ok(ifc_line.trim()[INTERFACE_LEN..].trim_end_matches('\n').to_string());
        }

        Ok("".to_string())
    }

    fn ipv4(&self, iface: &str) -> Result<String, Error> {
        run(Command::new("ipconfig").arg("getifaddr").arg(iface))
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
        todo!()
    }
}
