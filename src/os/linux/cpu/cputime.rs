use crate::linux::SysFs;
use crate::{
    util::{next, skip},
    Result,
};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::str::SplitAsciiWhitespace;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents time spent on processing instructions. Each field represents the amount of time,
/// measured in units of USER_HZ (1/100ths of a second on most architectures, use [`clock_tick`](crate::linux::cpu::clock_tick)
/// to get the precise value), that the system or the specific CPU spent in the state specified by the field name.
pub struct CpuTime {
    /// Time spent in user mode.
    pub user: u64,
    /// Time spent in user mode with low priority (nice).
    pub nice: u64,
    /// Time spent in system mode.
    pub system: u64,
    /// Time spent in the idle task.  This value should be USER_HZ times the second entry in
    /// the /proc/uptime pseudo-file.
    pub idle: u64,
    /// Time waiting for I/O to complete. Acording to the manual this value is not reliable.
    pub iowait: u64,
    /// Time servicing interrupts.
    pub irq: u64,
    /// Time servicing softirqs.
    pub softirq: u64,
    /// Stolen time, which is the time spent in other operating systems when running in a
    /// virtualized environment
    pub steal: u64,
    /// Time spent running a virtual CPU for guest operating systems under the control of
    /// the Linux kernel.
    pub guest: u64,
    /// Time spent running a niced guest (virtual CPU for guest operating systems under the
    /// control of the Linux kernel).
    pub guest_nice: u64,
}

impl CpuTime {
    pub fn total_time(&self) -> u64 {
        self.user
            + self.nice
            + self.system
            + self.idle
            + self.iowait
            + self.irq
            + self.softirq
            + self.steal
            + self.guest
            + self.guest_nice
    }

    pub fn idle_time(&self) -> u64 {
        self.idle
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
            steal: _next!(u64),
            guest: _next!(u64),
            guest_nice: _next!(u64),
        })
    }
}
