use super::{fs, run, Command, Error, MEM};

pub(crate) fn ip(iface: &str) -> Result<serde_json::Value, Error> {
    let mut _ip = Command::new("ip");
    let mut cmd = if iface == "" {
        _ip.arg("-j").arg("address").arg("show")
    } else {
        _ip.arg("-j").arg("address").arg("show").arg(&iface)
    };
    Ok(serde_json::from_str::<serde_json::Value>(&run(&mut cmd)?)
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn mem_extract(line: &str) -> Result<usize, Error> {
    Ok(fs::read_to_string(MEM)
        .map_err(|e| Error::FileReadError(MEM.to_string(), e.to_string()))?
        .split('\n')
        .filter(|l| l.starts_with(line))
        .collect::<String>()
        .split_ascii_whitespace()
        .skip(1)
        .take(1)
        .collect::<String>()
        .parse::<usize>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?
        * 1024 as usize)
}
