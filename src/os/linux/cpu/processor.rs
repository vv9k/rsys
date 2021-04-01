use crate::linux::cpu::{cores, Cores, CpuTime, BOGOMIPS, CACHE_SIZE, MODEL_NAME};
use crate::linux::{SysFs, SysPath};
use crate::{Error, Result};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// A structure representing host machine cpu
pub struct Processor {
    pub cores: Cores,
    pub model: String,
    pub cache_size: u64, // bytes
    pub bogomips: f32,
}

impl Processor {
    pub(crate) fn from_sys() -> Result<Processor> {
        let mut proc = Self::from_sys_path(&SysFs::Proc.join("cpuinfo"))?;
        proc.cores = cores()?;
        Ok(proc)
    }

    pub(crate) fn from_sys_path(path: &SysPath) -> Result<Processor> {
        let cpuinfo = path.read()?;
        let mut proc = Processor::default();
        for line in cpuinfo.lines() {
            dbg!(line);
            if line.starts_with(MODEL_NAME) {
                proc.model = Self::last_line_elem(line).to_string();
            } else if line.starts_with(BOGOMIPS) {
                proc.bogomips = Self::bogomips_from(line)?;
            } else if line.starts_with(CACHE_SIZE) {
                proc.cache_size = Self::cache_size_from(line)?;
            }
        }
        Ok(proc)
    }

    fn last_line_elem(line: &str) -> &str {
        line.split(':').last().unwrap_or_default().trim()
    }

    fn bogomips_from(line: &str) -> Result<f32> {
        f32::from_str(Self::last_line_elem(line)).map_err(|e| Error::InvalidInputError(line.to_string(), e.to_string()))
    }

    fn cache_size_from(line: &str) -> Result<u64> {
        u64::from_str(Self::last_line_elem(line).split_whitespace().next().unwrap_or_default())
            .map_err(|e| Error::InvalidInputError(line.to_string(), e.to_string()))
            .map(|v| v * 1024)
    }

    /// Returns core count of this processor
    pub fn core_count(&self) -> usize {
        self.cores.len()
    }

    pub fn cpu_time(&self) -> Result<Option<CpuTime>> {
        CpuTime::from_stat("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linux::mocks::CPUINFO;
    use std::{fs, io};

    #[test]
    fn creates_processor_from_cpuinfo() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        fs::write(dir.path().join("cpuinfo"), CPUINFO)?;

        let cpu = Processor {
            bogomips: 7_189.98,
            cache_size: 524_288,
            model: "AMD Ryzen 5 3600 6-Core Processor".to_string(),
            cores: Vec::new(),
        };

        assert_eq!(
            Ok(cpu),
            Processor::from_sys_path(&SysFs::Custom(dir.path().to_owned()).join("cpuinfo"))
        );

        dir.close()
    }
}
