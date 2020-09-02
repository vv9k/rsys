use super::*;

#[derive(Default)]
pub(crate) struct Linux {}
impl Linux {
    pub fn new() -> Self {
        Self::default()
    }
}
impl OsImpl for Linux {
    fn hostname(&self) -> Result<String, Error> {
        Ok(fs::read_to_string(HOSTNAME)
            .map_err(|e| Error::FileReadError(HOSTNAME.to_string(), e.to_string()))?
            .trim()
            .to_string())
    }

    fn uptime(&self) -> Result<u64, Error> {
        Ok(fs::read_to_string(UPTIME)
            .map_err(|e| Error::FileReadError(UPTIME.to_string(), e.to_string()))?
            .split_ascii_whitespace()
            .take(1)
            .collect::<String>()
            .parse::<f64>()
            .map_err(|e| Error::CommandParseError(e.to_string()))? as u64)
    }

    fn arch(&self) -> Result<String, Error> {
        run(Command::new("uname").arg("-m"))
    }

    fn cpu(&self) -> Result<String, Error> {
        Ok(fs::read_to_string(CPU)
            .map_err(|e| Error::FileReadError(CPU.to_string(), e.to_string()))?
            .split('\n')
            .filter(|l| l.starts_with(MODEL_NAME))
            .take(1)
            .collect::<String>()
            .split(':')
            .skip(1)
            .take(1)
            .collect::<String>()
            .trim()
            .to_string())
    }

    fn cpu_clock(&self) -> Result<f32, Error> {
        Ok(fs::read_to_string(CPU)
            .map_err(|e| Error::FileReadError(CPU.to_string(), e.to_string()))?
            .split('\n')
            .filter(|l| l.starts_with(CPU_CLOCK))
            .take(1)
            .collect::<String>()
            .split(':')
            .skip(1)
            .take(1)
            .collect::<String>()
            .trim()
            .parse::<f32>()
            .map_err(|e| Error::CommandParseError(e.to_string()))?)
    }

    fn cpu_cores(&self) -> Result<u16, Error> {
        Ok(fs::read_to_string(CPU)
            .map_err(|e| Error::FileReadError(CPU.to_string(), e.to_string()))?
            .split('\n')
            .filter(|l| l.starts_with(CPU_CORES))
            .take(1)
            .collect::<String>()
            .split(':')
            .skip(1)
            .take(1)
            .collect::<String>()
            .trim()
            .parse::<u16>()
            .map_err(|e| Error::CommandParseError(e.to_string()))?)
    }

    fn logical_cores(&self) -> Result<u16, Error> {
        Ok(fs::read_to_string(CPU)
            .map_err(|e| Error::FileReadError(CPU.to_string(), e.to_string()))?
            .split('\n')
            .filter(|l| l.starts_with(SIBLINGS))
            .take(1)
            .collect::<String>()
            .split(':')
            .skip(1)
            .take(1)
            .collect::<String>()
            .trim()
            .parse::<u16>()
            .map_err(|e| Error::CommandParseError(e.to_string()))?)
    }

    fn memory(&self) -> Result<usize, Error> {
        Ok(fs::read_to_string(MEM)
            .map_err(|e| Error::FileReadError(MEM.to_string(), e.to_string()))?
            .split('\n')
            .filter(|l| l.starts_with(TOTAL_MEM))
            .collect::<String>()
            .split_ascii_whitespace()
            .skip(1)
            .take(1)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|e| Error::CommandParseError(e.to_string()))? as usize)
    }

    fn swap(&self) -> Result<usize, Error> {
        Ok(fs::read_to_string(MEM)
            .map_err(|e| Error::FileReadError(MEM.to_string(), e.to_string()))?
            .split('\n')
            .filter(|l| l.starts_with(TOTAL_SWAP))
            .collect::<String>()
            .split_ascii_whitespace()
            .skip(1)
            .take(1)
            .collect::<String>()
            .parse::<usize>()
            .map_err(|e| Error::CommandParseError(e.to_string()))? as usize)
    }

    fn default_iface(&self) -> Result<String, Error> {
        let mut cmd = Command::new("route");
        Ok(run(&mut cmd)?
            .split('\n')
            .filter(|l| l.starts_with("default"))
            .collect::<String>()
            .split_ascii_whitespace()
            .last()
            .ok_or_else(|| Error::CommandParseError("output of route command was invalid".to_string()))?
            .to_string())
    }

    fn ipv4(&self, iface: &str) -> Result<String, Error> {
        let out = ip(&iface)?;
        let ip = &out[0]["addr_info"][0]["local"];
        if ip.is_string() {
            // It's ok to unwrap here because we know it's a string
            return Ok(ip.as_str().map(|s| s.to_string()).unwrap());
        }

        Err(Error::CommandParseError(format!(
            "ip address '{:?}' was not a string",
            ip
        )))
    }

    fn ipv6(&self, _iface: &str) -> Result<String, Error> {
        todo!()
    }

    fn mac(&self, iface: &str) -> Result<String, Error> {
        let out = ip(&iface)?;
        let mac = &out[0]["address"];
        if mac.is_string() {
            // It's ok to unwrap here because we know it's a string
            return Ok(mac.as_str().map(|s| s.to_string()).unwrap());
        }

        Err(Error::CommandParseError(format!(
            "mac address '{:?}' was not a string",
            mac
        )))
    }

    fn interfaces(&self) -> Result<Vec<String>, Error> {
        let out = ip("")?;
        if !out.is_array() {
            return Err(Error::CommandParseError("invalid 'ip' command output".to_string()));
        }

        // It's ok to unwrap here because we check that out is an array and all non-string values are filtered out
        Ok(out
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v["ifname"].is_string())
            .map(|v| v["ifname"].as_str().unwrap().to_string())
            .collect())
    }

    fn domainname(&self) -> Result<String, Error> {
        Ok(fs::read_to_string(DOMAINNAME)
            .map_err(|e| Error::FileReadError(DOMAINNAME.to_string(), e.to_string()))?
            .trim()
            .to_string())
    }
}
