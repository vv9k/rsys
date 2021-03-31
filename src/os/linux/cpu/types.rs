#[cfg(test)]
use super::CPUINFO;
use super::{cores, SysFs, SysPath, BOGOMIPS, CACHE_SIZE, MODEL_NAME};
use crate::{
    util::{next, skip},
    Error, Result,
};
use std::str::{FromStr, SplitAsciiWhitespace};
pub type Cores = Vec<Core>;
#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

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
        let mut proc = Self::from_sys_path(SysFs::Proc.join("cpuinfo"))?;
        proc.cores = cores()?;
        Ok(proc)
    }

    pub(crate) fn from_sys_path(path: SysPath) -> Result<Processor> {
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct CpuTime {
    pub user: u64,
    pub nice: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub irq: u64,
    pub softirq: u64,
}
impl CpuTime {
    fn from_stat_line(stat: &str) -> Result<CpuTime> {
        let mut elems = stat.split_ascii_whitespace();

        Ok(CpuTime {
            user: next::<u64, SplitAsciiWhitespace>(skip(1, &mut elems), &stat)?,
            nice: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            system: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            idle: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            iowait: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            irq: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            softirq: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
        })
    }

    pub(crate) fn from_stat(id: &str) -> Result<Option<CpuTime>> {
        let name = format!("cpu{}", id);
        for line in SysFs::Proc.join("stat").read()?.lines() {
            if line.starts_with(&name) {
                return Ok(Some(CpuTime::from_stat_line(line)?));
            }
        }
        Ok(None)
    }

    pub fn total_time(&self) -> u64 {
        self.user + self.nice + self.system + self.idle + self.iowait + self.irq + self.softirq
    }

    pub fn idle_time(&self) -> u64 {
        self.idle
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
        Self::from_sys_path(
            SysFs::Sys
                .join("devices")
                .join("system")
                .join("cpu")
                .join(format!("cpu{}", id)),
        )
    }

    fn from_sys_path(p: SysPath) -> Result<Core> {
        let freq_p = p.clone().join("cpufreq");
        let id = Core::core_id(p)?;
        Ok(Core {
            id,
            min_freq: Core::frequency(freq_p.clone(), Frequency::Minimal)?,
            cur_freq: Core::frequency(freq_p.clone(), Frequency::Current)?,
            max_freq: Core::frequency(freq_p, Frequency::Maximal)?,
        })
    }

    fn core_id(p: SysPath) -> Result<u32> {
        p.extend(&["topology", "core_id"]).read_as::<u32>()
    }

    fn frequency(p: SysPath, which: Frequency) -> Result<u64> {
        let mut new_p;
        match which {
            Frequency::Minimal => new_p = p.clone().join("scaling_min_freq"),
            Frequency::Current => new_p = p.clone().join("scaling_cur_freq"),
            Frequency::Maximal => new_p = p.clone().join("scaling_max_freq"),
        };
        if !new_p.clone().path().exists() {
            match which {
                Frequency::Minimal => new_p = p.join("cpuinfo_min_freq"),
                Frequency::Current => new_p = p.join("cpuinfo_cur_freq"),
                Frequency::Maximal => new_p = p.join("cpuinfo_max_freq"),
            };
        }

        if !new_p.clone().path().exists() {
            return Ok(0);
        }

        // Value is in KHz so we multiply it by 1000
        new_p.read_as::<u64>().map(|f| f * 1000)
    }

    /// Updates all frequencies of this core to currently available values
    pub fn update(&mut self) -> Result<()> {
        let path = SysFs::Sys
            .join("devices")
            .join("system")
            .join("cpu")
            .join(format!("cpu{}", self.id));
        self.min_freq = Core::frequency(path.clone(), Frequency::Minimal)?;
        self.cur_freq = Core::frequency(path.clone(), Frequency::Current)?;
        self.max_freq = Core::frequency(path, Frequency::Maximal)?;

        Ok(())
    }

    pub fn cpu_time(&self) -> Result<Option<CpuTime>> {
        CpuTime::from_stat(&format!("{}", self.id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io};

    #[test]
    fn creates_core_scaling_frequency() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let freq_p = dir.path().join("cpufreq");
        fs::create_dir(freq_p.as_path())?;

        let topology_p = dir.path().join("topology");
        fs::create_dir(topology_p.as_path())?;
        fs::write(topology_p.join("core_id"), b"1")?;

        fs::write(freq_p.join("scaling_min_freq"), b"2200000")?;
        fs::write(freq_p.join("scaling_cur_freq"), b"3443204")?;
        fs::write(freq_p.join("scaling_max_freq"), b"3600000")?;

        let core = Core {
            id: 1,
            min_freq: 2_200_000_000,
            cur_freq: 3_443_204_000,
            max_freq: 3_600_000_000,
        };

        assert_eq!(
            Ok(core),
            Core::from_sys_path(SysFs::Custom(dir.path().to_owned()).as_syspath())
        );

        dir.close()
    }

    #[test]
    fn creates_core_fallback_cpuinfo() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let freq_p = dir.path().join("cpufreq");
        fs::create_dir(freq_p.as_path())?;

        let topology_p = dir.path().join("topology");
        fs::create_dir(topology_p.as_path())?;
        fs::write(topology_p.join("core_id"), b"1")?;

        fs::write(freq_p.join("cpuinfo_min_freq"), b"2200000")?;
        fs::write(freq_p.join("cpuinfo_cur_freq"), b"3443204")?;
        fs::write(freq_p.join("cpuinfo_max_freq"), b"3600000")?;

        let core = Core {
            id: 1,
            min_freq: 2_200_000_000,
            cur_freq: 3_443_204_000,
            max_freq: 3_600_000_000,
        };

        assert_eq!(
            Ok(core),
            Core::from_sys_path(SysFs::Custom(dir.path().to_owned()).as_syspath())
        );

        dir.close()
    }

    #[test]
    fn parses_cputime_from_stat() {
        let line = "cpu0 12902 26 1888 731468 332 224 183 0 0 0";
        let time = CpuTime {
            user: 12_902,
            nice: 26,
            system: 1_888,
            idle: 731_468,
            iowait: 332,
            irq: 224,
            softirq: 183,
        };
        assert_eq!(Ok(time), CpuTime::from_stat_line(&line));
    }

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
            Processor::from_sys_path(SysFs::Custom(dir.path().to_owned()).join("cpuinfo"))
        );

        dir.close()
    }
}
