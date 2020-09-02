use super::{run, Error, ProcPath};
use std::fmt::Display;
use std::process::Command;
use std::str::FromStr;

pub(crate) const MODEL_NAME: &str = "model name";
pub(crate) const CPU_CORES: &str = "cpu cores";
pub(crate) const SIBLINGS: &str = "siblings";
pub(crate) const CPU_CLOCK: &str = "cpu MHz";
pub(crate) const MEM_TOTAL: &str = "MemTotal:";
pub(crate) const MEM_FREE: &str = "MemAvailable:";
pub(crate) const SWAP_TOTAL: &str = "SwapTotal:";
pub(crate) const SWAP_FREE: &str = "SwapFree:";

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
    Ok(ProcPath::MemInfo
        .read()?
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

pub(crate) fn cpuinfo_extract<T: FromStr>(line: &str) -> Result<T, Error>
where
    <T as FromStr>::Err: Display,
{
    Ok(ProcPath::CpuInfo
        .read()?
        .split('\n')
        .filter(|l| l.starts_with(line))
        .take(1)
        .collect::<String>()
        .split(':')
        .skip(1)
        .take(1)
        .collect::<String>()
        .trim()
        .parse::<T>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}
