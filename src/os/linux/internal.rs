use super::{run, Error, SysPath};
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
    _mem_extract(&SysPath::ProcMemInfo.read()?, &line)
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
    _cpuinfo_extract(&SysPath::ProcCpuInfo.read()?, &line)
}

// Parses out major and minor number from str like '8:1'
// and returns a tuple (8, 1)
pub(crate) fn parse_maj_min(dev: &str) -> Option<(u32, u32)> {
    let mut elems = dev.split(':');

    if let Some(maj) = elems.next() {
        if let Some(min) = elems.next() {
            if let Ok(maj) = maj.trim().parse::<u32>() {
                if let Ok(min) = min.trim().parse::<u32>() {
                    return Some((maj, min));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_meminfo() {
        assert_eq!(_mem_extract(MEMINFO, MEM_TOTAL), Ok(16712671232));
        assert_eq!(_mem_extract(MEMINFO, MEM_FREE), Ok(14993084416));
        assert_eq!(_mem_extract(MEMINFO, SWAP_TOTAL), Ok(0));
        assert_eq!(_mem_extract(MEMINFO, SWAP_FREE), Ok(0));
    }
    #[test]
    fn extracts_cpuinfo() {
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, CPU_CORES), Ok(6));
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, SIBLINGS), Ok(12));
        assert_eq!(_cpuinfo_extract::<f32>(CPUINFO, CPU_CLOCK), Ok(2053.971));
    }
    #[test]
    fn parses_maj_min() {
        assert_eq!(parse_maj_min("X:5"), None);
        assert_eq!(parse_maj_min("1:Y"), None);
        assert_eq!(parse_maj_min("rand:"), None);
        assert_eq!(parse_maj_min(":xx"), None);
        assert_eq!(parse_maj_min("xxx"), None);
        assert_eq!(parse_maj_min("8:1"), Some((8, 1)))
    }
}
