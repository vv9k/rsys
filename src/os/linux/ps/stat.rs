use crate::linux::ps::{stat_process, ProcessState};
use crate::linux::SysPath;
use crate::{
    util::{next, skip},
    Result,
};

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::str::SplitAsciiWhitespace;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a process stat from /proc/[pid]/stat
pub struct ProcessStat {
    pub pid: i32,
    pub name: String,
    pub state: ProcessState,
    pub ppid: i32,
    pub pgrp: i32,
    pub session: i32,
    pub tty_nr: i32,
    pub utime: u64,
    pub stime: u64,
    pub cutime: i64,
    pub cstime: i64,
    pub priority: i32,
    pub nice: i32,
    pub num_threads: i32,
    pub itrealvalue: i32,
    pub starttime: u64,
    pub vsize: u64,
    pub rss: i32,
    pub rsslim: u64,
    pub nswap: u32,
    pub cnswap: u32,
    pub guest_time: u32,
    pub cguest_time: u32,
    pub processor: u32,
}

impl ProcessStat {
    pub(crate) fn from_stat(stat: &str) -> Result<ProcessStat> {
        let mut elems = stat.split_ascii_whitespace();

        Ok(ProcessStat {
            pid: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            name: {
                let mut s = next::<String, SplitAsciiWhitespace>(&mut elems, &stat)?;
                // Handle case where cmdline parameter contains whitespace.
                // for example: `(tmux: client)` is split into two parts
                // that have to be glued together.
                if !s.ends_with(')') {
                    s.push(' ');
                    s.push_str(&next::<String, SplitAsciiWhitespace>(&mut elems, &stat)?);
                }
                s
            },
            state: ProcessState::from(next::<String, SplitAsciiWhitespace>(&mut elems, &stat)?.as_str()),
            ppid: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            pgrp: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            session: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            tty_nr: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            utime: next::<u64, SplitAsciiWhitespace>(skip(6, &mut elems), &stat)?,
            stime: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            cutime: next::<i64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            cstime: next::<i64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            priority: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            nice: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            num_threads: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            itrealvalue: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            starttime: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            vsize: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            rss: next::<i32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            rsslim: next::<u64, SplitAsciiWhitespace>(&mut elems, &stat)?,
            nswap: next::<u32, SplitAsciiWhitespace>(skip(10, &mut elems), &stat)?,
            cnswap: next::<u32, SplitAsciiWhitespace>(&mut elems, &stat)?,
            processor: next::<u32, SplitAsciiWhitespace>(skip(1, &mut elems), &stat)?,
            guest_time: next::<u32, SplitAsciiWhitespace>(skip(3, &mut elems), &stat)?,
            cguest_time: next::<u32, SplitAsciiWhitespace>(&mut elems, &stat)?,
        })
    }

    pub(crate) fn from_sys_path(path: SysPath) -> Result<ProcessStat> {
        ProcessStat::from_stat(&path.join("stat").read()?)
    }

    pub fn update(&mut self) -> Result<()> {
        let p = stat_process(self.pid)?;
        self.pid = p.pid;
        self.name = p.name;
        self.state = p.state;
        self.ppid = p.ppid;
        self.pgrp = p.pgrp;
        self.session = p.session;
        self.tty_nr = p.tty_nr;
        self.utime = p.utime;
        self.stime = p.stime;
        self.cutime = p.cutime;
        self.cstime = p.cstime;
        self.priority = p.priority;
        self.nice = p.nice;
        self.num_threads = p.num_threads;
        self.itrealvalue = p.itrealvalue;
        self.starttime = p.starttime;
        self.vsize = p.vsize;
        self.rss = p.rss;
        self.rsslim = p.rsslim;
        self.nswap = p.nswap;
        self.cnswap = p.cnswap;
        self.guest_time = p.guest_time;
        self.cguest_time = p.cguest_time;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linux::mocks::{PROCESS_STAT, PROCESS_STAT_WHITESPACE_NAME};

    #[test]
    fn parses_process_stat() {
        let process = ProcessStat {
            pid: 69035,
            name: "(alacritty)".to_string(),
            state: ProcessState::Sleeping,
            ppid: 1,
            pgrp: 69035,
            session: 69035,
            tty_nr: 0,
            utime: 3977,
            stime: 293,
            cutime: 0,
            cstime: 0,
            priority: 20,
            nice: 0,
            num_threads: 26,
            itrealvalue: 0,
            starttime: 967628,
            vsize: 2158927872,
            rss: 45316,
            rsslim: 18446744073709551615,
            nswap: 0,
            cnswap: 0,
            guest_time: 0,
            cguest_time: 0,
            processor: 6,
        };
        assert_eq!(ProcessStat::from_stat(PROCESS_STAT), Ok(process))
    }

    #[test]
    fn parses_process_stat_with_whitespace_in_name() {
        let process = ProcessStat {
            pid: 1483,
            name: "(tmux: server)".to_string(),
            state: ProcessState::Sleeping,
            ppid: 1,
            pgrp: 1483,
            session: 1483,
            tty_nr: 0,
            utime: 440,
            stime: 132,
            cutime: 0,
            cstime: 0,
            priority: 20,
            nice: 0,
            num_threads: 1,
            itrealvalue: 0,
            starttime: 8224,
            vsize: 12197888,
            rss: 1380,
            rsslim: 18446744073709551615,
            nswap: 0,
            cnswap: 0,
            guest_time: 0,
            cguest_time: 0,
            processor: 6,
        };
        assert_eq!(ProcessStat::from_stat(PROCESS_STAT_WHITESPACE_NAME), Ok(process))
    }
}
