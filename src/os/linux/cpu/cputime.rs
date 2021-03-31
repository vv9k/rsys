#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::str::SplitAsciiWhitespace;

use crate::linux::SysFs;
use crate::{
    util::{next, skip},
    Result,
};

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
    pub(crate) fn from_stat_line(stat: &str) -> Result<CpuTime> {
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
