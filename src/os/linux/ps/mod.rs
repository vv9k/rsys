#[cfg(test)]
use super::mocks::{PROCESS_STAT, PROCESS_STAT_WHITESPACE_NAME};
use super::{Error, SysPath};
use crate::util::{next, skip};
use std::{fs, str::SplitAsciiWhitespace};

pub type Processes = Vec<Process>;

//#TODO: Add more states
#[derive(Debug, Eq, PartialEq)]
pub enum ProcessState {
    Sleeping,
    Zombie,
    Unknown,
}
impl From<&str> for ProcessState {
    fn from(s: &str) -> Self {
        use self::ProcessState::*;
        match s.chars().next() {
            Some('S') => Sleeping,
            Some('Z') => Zombie,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
/// Represents a process from /proc/[pid]/stat
pub struct Process {
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
}

impl Process {
    pub(crate) fn from_stat(stat: &str) -> Result<Process, Error> {
        let mut elems = stat.split_ascii_whitespace();

        Ok(Process {
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
            guest_time: next::<u32, SplitAsciiWhitespace>(skip(5, &mut elems), &stat)?,
            cguest_time: next::<u32, SplitAsciiWhitespace>(&mut elems, &stat)?,
        })
    }
}

/// Returns detailed Process information parsed from /proc/[pid]/stat
pub fn stat_process(pid: i32) -> Result<Process, Error> {
    Process::from_stat(&SysPath::ProcPidStat(pid).read()?)
}

/// Returns a list of pids read from /proc
pub fn pids() -> Result<Vec<i32>, Error> {
    let path = SysPath::Proc.path();
    let mut pids = Vec::new();
    for _entry in
        fs::read_dir(&path).map_err(|e| Error::FileReadError(path.to_string_lossy().to_string(), e.to_string()))?
    {
        if let Ok(entry) = _entry {
            let filename = entry.file_name();
            let sfilename = filename.as_os_str().to_string_lossy();
            if sfilename.chars().all(|c| c.is_digit(10)) {
                pids.push(
                    sfilename
                        .parse::<i32>()
                        .map_err(|e| Error::InvalidInputError(sfilename.to_string(), e.to_string()))?,
                );
            }
        }
    }
    Ok(pids)
}

/// Returns all processes currently seen in /proc parsed as Processes
pub fn processes() -> Result<Processes, Error> {
    let mut _pids = Vec::new();
    for pid in pids()? {
        _pids.push(stat_process(pid)?);
    }

    Ok(_pids)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_process_from_stat() {
        let process = Process {
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
        };
        assert_eq!(Process::from_stat(PROCESS_STAT), Ok(process))
    }

    #[test]
    fn parses_process_with_whitespace_from_stat() {
        let process = Process {
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
        };
        assert_eq!(Process::from_stat(PROCESS_STAT_WHITESPACE_NAME), Ok(process))
    }
}
