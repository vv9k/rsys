pub(crate) mod types;

#[cfg(test)]
use super::mocks::CPUINFO;
pub(crate) use super::SysPath;
use crate::{Error, Result};
use std::{fmt::Display, str::FromStr};
pub use types::*;

pub(crate) const MODEL_NAME: &str = "model name";
pub(crate) const CACHE_SIZE: &str = "cache size";
pub(crate) const BOGOMIPS: &str = "bogomips";
pub(crate) const CPU_CORES: &str = "cpu cores";
pub(crate) const SIBLINGS: &str = "siblings";
pub(crate) const CPU_CLOCK: &str = "cpu MHz";

fn _cpuinfo_extract<T: FromStr>(out: &str, line: &str) -> Result<T>
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

pub(crate) fn cpuinfo_extract<T: FromStr>(line: &str) -> Result<T>
where
    <T as FromStr>::Err: Display,
{
    _cpuinfo_extract(&SysPath::ProcCpuInfo.read()?, &line)
}

fn core_ids() -> Result<Vec<u32>> {
    let mut core_ids = Vec::new();
    for entry in SysPath::SysDevicesSystemCpu().read_dir()? {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if !file_name.starts_with("cpu") {
                continue;
            }
            if let Some(digits) = file_name.split("cpu").last() {
                if let Some(digit) = digits.chars().next() {
                    if !digit.is_digit(10) {
                        continue;
                    }

                    core_ids.push(
                        u32::from_str_radix(digits, 10)
                            .map_err(|e| Error::InvalidInputError(file_name, e.to_string()))?,
                    );
                }
            }
        }
    }

    Ok(core_ids)
}

/// Returns the name of first seen cpu in /proc/cpuinfo
pub fn cpu() -> Result<String> {
    cpuinfo_extract::<String>(MODEL_NAME)
}

/// Returns cpu clock of first core in /proc/cpuinfo file.
pub fn cpu_clock() -> Result<f32> {
    cpuinfo_extract::<f32>(CPU_CLOCK)
}

/// Returns total cpu cores available.
pub fn cpu_cores() -> Result<u16> {
    cpuinfo_extract::<u16>(CPU_CORES)
}

/// Returns total logical cores available.
pub fn logical_cores() -> Result<u16> {
    cpuinfo_extract::<u16>(SIBLINGS)
}

/// Returns Core objects with frequencies
pub fn cores() -> Result<Cores> {
    let mut cores = Vec::new();
    for id in core_ids()? {
        cores.push(Core::from_sys(id)?);
    }

    Ok(cores)
}

/// Returns a Processor object containing gathered information
/// about host machine processor.
pub fn processor() -> Result<Processor> {
    Processor::from_sys()
}

pub fn clock_tick() -> Result<Option<i64>> {
    nix::unistd::sysconf(nix::unistd::SysconfVar::CLK_TCK)
        .map_err(|e| Error::FfiError("getting clock tick of cpu".to_string(), e.to_string()))
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
