use crate::linux::cpu::cputime::CpuTime;
use crate::linux::{SysFs, SysPath};
use crate::Result;

pub type Cores = Vec<Core>;

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
    /// Updates all frequencies of this core to currently available values
    pub fn update(&mut self) -> Result<()> {
        let path = SysFs::Sys.join("devices/system/cpu").join(format!("cpu{}", self.id));
        self.min_freq = Core::frequency(&path, Frequency::Minimal)?;
        self.cur_freq = Core::frequency(&path, Frequency::Current)?;
        self.max_freq = Core::frequency(&path, Frequency::Maximal)?;

        Ok(())
    }

    /// Returns the cpu time spent by this core
    pub fn cpu_time(&self) -> Result<Option<CpuTime>> {
        CpuTime::from_stat(&format!("{}", self.id))
    }

    pub(crate) fn from_sys(id: u32) -> Result<Core> {
        Self::from_sys_path(&SysFs::Sys.join("devices/system/cpu").join(format!("cpu{}", id)))
    }

    fn from_sys_path(p: &SysPath) -> Result<Core> {
        let freq_p = p.extend("cpufreq");
        let id = Core::core_id(p)?;
        Ok(Core {
            id,
            min_freq: Core::frequency(&freq_p, Frequency::Minimal)?,
            cur_freq: Core::frequency(&freq_p, Frequency::Current)?,
            max_freq: Core::frequency(&freq_p, Frequency::Maximal)?,
        })
    }

    fn core_id(p: &SysPath) -> Result<u32> {
        p.extend("topology/core_id").read_as::<u32>()
    }

    fn frequency(p: &SysPath, which: Frequency) -> Result<u64> {
        let mut new_p;
        match which {
            Frequency::Minimal => new_p = p.extend("scaling_min_freq"),
            Frequency::Current => new_p = p.extend("scaling_cur_freq"),
            Frequency::Maximal => new_p = p.extend("scaling_max_freq"),
        };
        if !new_p.as_path().exists() {
            match which {
                Frequency::Minimal => new_p = p.extend("cpuinfo_min_freq"),
                Frequency::Current => new_p = p.extend("cpuinfo_cur_freq"),
                Frequency::Maximal => new_p = p.extend("cpuinfo_max_freq"),
            };
        }

        if !new_p.as_path().exists() {
            return Ok(0);
        }

        // Value is in KHz so we multiply it by 1000
        new_p.read_as::<u64>().map(|f| f * 1000)
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
            Core::from_sys_path(&SysFs::Custom(dir.path().to_owned()).to_syspath())
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
            Core::from_sys_path(&SysFs::Custom(dir.path().to_owned()).to_syspath())
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
            steal: 0,
            guest: 0,
            guest_nice: 0,
        };
        assert_eq!(Ok(time), CpuTime::from_stat_line(&line));
    }
}
