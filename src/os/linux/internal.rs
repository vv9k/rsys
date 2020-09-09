use super::{run, Error, ProcPath};
use std::fmt::Display;
use std::process::Command;
use std::str::FromStr;

#[cfg(test)]
use super::mocks::*;

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

fn _mem_extract(out: &str, line: &str) -> Result<usize, Error> {
    Ok(out
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

pub(crate) fn mem_extract(line: &str) -> Result<usize, Error> {
    _mem_extract(&ProcPath::MemInfo.read()?, &line)
}

fn _cpuinfo_extract<T: FromStr>(out: &str, line: &str) -> Result<T, Error>
where
    <T as FromStr>::Err: Display,
{
    Ok(out
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

pub(crate) fn cpuinfo_extract<T: FromStr>(line: &str) -> Result<T, Error>
where
    <T as FromStr>::Err: Display,
{
    _cpuinfo_extract(&ProcPath::CpuInfo.read()?, &line)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extract_meminfo() {
        assert_eq!(_mem_extract(MEMINFO, MEM_TOTAL).unwrap(), 16712671232);
        assert_eq!(_mem_extract(MEMINFO, MEM_FREE).unwrap(), 14993084416);
        assert_eq!(_mem_extract(MEMINFO, SWAP_TOTAL).unwrap(), 0);
        assert_eq!(_mem_extract(MEMINFO, SWAP_FREE).unwrap(), 0);
    }
    #[test]
    fn extract_cpuinfo() {
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, CPU_CORES).unwrap(), 6);
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, SIBLINGS).unwrap(), 12);
        assert_eq!(_cpuinfo_extract::<f32>(CPUINFO, CPU_CLOCK).unwrap(), 2053.971);
    }
}
