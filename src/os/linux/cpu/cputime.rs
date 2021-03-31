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

        macro_rules! _next {
            ($t:tt) => {
                next::<$t, SplitAsciiWhitespace>(&mut elems, &stat)?
            };
        }

        Ok(CpuTime {
            user: next::<u64, SplitAsciiWhitespace>(skip(1, &mut elems), &stat)?,
            nice: _next!(u64),
            system: _next!(u64),
            idle: _next!(u64),
            iowait: _next!(u64),
            irq: _next!(u64),
            softirq: _next!(u64),
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
