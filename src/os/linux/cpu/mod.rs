#[cfg(test)]
use super::mocks::CPUINFO;
use super::{Error, SysPath};
use std::fmt::Display;
use std::str::FromStr;

pub(crate) const MODEL_NAME: &str = "model name";
pub(crate) const CPU_CORES: &str = "cpu cores";
pub(crate) const SIBLINGS: &str = "siblings";
pub(crate) const CPU_CLOCK: &str = "cpu MHz";

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

/// Returns the name of first seen cpu in /proc/cpuinfo
pub fn cpu() -> Result<String, Error> {
    cpuinfo_extract::<String>(MODEL_NAME)
}

/// Returns cpu clock of first core in /proc/cpuinfo file.
pub fn cpu_clock() -> Result<f32, Error> {
    cpuinfo_extract::<f32>(CPU_CLOCK)
}

/// Returns total cpu cores available.
pub fn cpu_cores() -> Result<u16, Error> {
    cpuinfo_extract::<u16>(CPU_CORES)
}

/// Returns total logical cores available.
pub fn logical_cores() -> Result<u16, Error> {
    cpuinfo_extract::<u16>(SIBLINGS)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_cpuinfo() {
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, CPU_CORES), Ok(6));
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, SIBLINGS), Ok(12));
        assert_eq!(_cpuinfo_extract::<f32>(CPUINFO, CPU_CLOCK), Ok(2053.971));
    }
}
