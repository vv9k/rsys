use super::{cores, SysPath, BOGOMIPS, CACHE_SIZE, MODEL_NAME};
use crate::{Error, Result};
use std::str::FromStr;
pub type Cores = Vec<Core>;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
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
        let cpuinfo = SysPath::ProcCpuInfo.read()?;
        let mut proc = Processor::default();
        proc.cores = cores()?;
        for line in cpuinfo.lines() {
            if line.starts_with(MODEL_NAME) {
                proc.model = Self::last_line_elem(line).to_string();
            } else if line.starts_with(BOGOMIPS) {
                proc.bogomips = Self::bogomips_from(line)?;
            } else if line.starts_with(CACHE_SIZE) {
                proc.cache_size = Self::cache_size_from(line)?;
                break;
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
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a virtual core in a cpu
pub struct Core {
    pub id: u32,
    pub min_freq: u64,
    pub cur_freq: u64,
    pub max_freq: u64,
}

enum Frequency {
    Minimal,
    Current,
    Maximal,
}

impl Core {
    pub(crate) fn from_sys(id: u32) -> Result<Core> {
        Ok(Core {
            id,
            min_freq: Core::frequency(id, Frequency::Minimal)?,
            cur_freq: Core::frequency(id, Frequency::Current)?,
            max_freq: Core::frequency(id, Frequency::Maximal)?,
        })
    }

    fn frequency(id: u32, which: Frequency) -> Result<u64> {
        let mut p = SysPath::SysDevicesSystemCpuCore(id).extend(&["cpufreq"]);
        match which {
            Frequency::Minimal => p = p.extend(&["scaling_min_freq"]),
            Frequency::Current => p = p.extend(&["scaling_cur_freq"]),
            Frequency::Maximal => p = p.extend(&["scaling_max_freq"]),
        };
        if !p.clone().path().exists() {
            p = SysPath::SysDevicesSystemCpuCore(id).extend(&["cpufreq"]);
            match which {
                Frequency::Minimal => p = p.extend(&["cpuinfo_min_freq"]),
                Frequency::Current => p = p.extend(&["cpuinfo_cur_freq"]),
                Frequency::Maximal => p = p.extend(&["cpuinfo_max_freq"]),
            };
        }

        p.read_as::<u64>()
    }

    /// Updates all frequencies of this core to currently available values
    pub fn update(&mut self) -> Result<()> {
        self.min_freq = Core::frequency(self.id, Frequency::Minimal)?;
        self.cur_freq = Core::frequency(self.id, Frequency::Current)?;
        self.max_freq = Core::frequency(self.id, Frequency::Maximal)?;

        Ok(())
    }
}
