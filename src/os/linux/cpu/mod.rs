pub(crate) mod cores;
pub(crate) mod processor;
pub(crate) mod time;

pub use cores::*;
pub use processor::*;
pub use time::*;

use crate::linux::{SysFs, SysPath};
use crate::{Error, Result};

use std::{fmt::Display, str::FromStr};

const MODEL_NAME: &str = "model name";
const CACHE_SIZE: &str = "cache size";
const BOGOMIPS: &str = "bogomips";
const CPU_CORES: &str = "cpu cores";
const SIBLINGS: &str = "siblings";
const CPU_CLOCK: &str = "cpu MHz";

//################################################################################
// Public
//################################################################################

/// Returns the name of first seen cpu in /proc/cpuinfo
pub fn model() -> Result<String> {
    cpuinfo_extract::<String>(MODEL_NAME)
}

/// Returns cpu clock of first core in /proc/cpuinfo file.
pub fn clock() -> Result<f32> {
    cpuinfo_extract::<f32>(CPU_CLOCK)
}

/// Returns total cpu cores available.
pub fn core_count() -> Result<u16> {
    cpuinfo_extract::<u16>(CPU_CORES)
}

/// Returns total logical cores available.
pub fn logical_cores() -> Result<u16> {
    cpuinfo_extract::<u16>(SIBLINGS)
}

/// Returns Core objects with frequencies
pub fn cores() -> Result<Cores> {
    let mut cores = Vec::new();
    for id in core_ids(SysFs::Sys.join("devices/system/cpu"))? {
        cores.push(Core::from_sys(id)?);
    }

    Ok(cores)
}

/// Returns a Processor object containing gathered information
/// about host machine processor.
pub fn processor() -> Result<Processor> {
    Processor::from_sys()
}

//################################################################################
// Internal
//################################################################################

fn cpuinfo_extract<T: FromStr>(line: &str) -> Result<T>
where
    <T as FromStr>::Err: Display,
{
    _cpuinfo_extract(&SysFs::Proc.join("cpuinfo").read()?, line)
}

fn _cpuinfo_extract<T: FromStr>(out: &str, line: &str) -> Result<T>
where
    <T as FromStr>::Err: Display,
{
    out.split('\n')
        .find(|l| l.starts_with(line))
        .map(|out_line| {
            out_line
                .split(':')
                .skip(1)
                .take(1)
                .next()
                .map(|s| {
                    s.trim()
                        .parse::<T>()
                        .map_err(|e| Error::InvalidInputError(out_line.to_string(), e.to_string()))
                })
                .ok_or_else(|| Error::InvalidInputError(line.to_string(), "missing line from cpuinfo".to_string()))
        })
        .ok_or_else(|| Error::InvalidInputError(line.to_string(), "missing line from cpuinfo".to_string()))??
}

fn core_ids(path: SysPath) -> Result<Vec<u32>> {
    let mut core_ids = Vec::new();
    for entry in path.read_dir()?.flatten() {
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
                    digits
                        .parse::<u32>()
                        .map_err(|e| Error::InvalidInputError(file_name, e.to_string()))?,
                );
            }
        }
    }

    Ok(core_ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linux::mocks::CPUINFO;
    use std::{fs::File, io};
    #[test]
    fn extracts_cpuinfo() {
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, CPU_CORES).unwrap(), 6);
        assert_eq!(_cpuinfo_extract::<u32>(CPUINFO, SIBLINGS).unwrap(), 12);
        assert!((_cpuinfo_extract::<f32>(CPUINFO, CPU_CLOCK).unwrap() - 2053.971_f32).abs() < f32::EPSILON);
    }

    #[test]
    fn finds_core_ids() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut ids = Vec::new();
        for id in 0..16 {
            File::create(dir.path().join(format!("cpu{}", id)))?;
            ids.push(id);
        }

        let mut out = core_ids(SysFs::Custom(dir.path().to_owned()).into_syspath()).unwrap();
        out.sort_unstable();

        assert_eq!(ids, out);

        dir.close()
    }
}
