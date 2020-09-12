#[cfg(test)]
use super::mocks::{NET_DEV, PROCESS_STAT, PROCESS_STAT_WHITESPACE_NAME};
use super::Error;
use std::any::type_name;
use std::str::SplitAsciiWhitespace;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct MountPoint {
    volume: String,
    path: String,
    voltype: String,
    options: String,
}
impl MountPoint {
    fn new(volume: &str, path: &str, voltype: &str, options: &str) -> MountPoint {
        MountPoint {
            volume: volume.to_string(),
            path: path.to_string(),
            voltype: voltype.to_string(),
            options: options.to_string(),
        }
    }
    pub(crate) fn from_line(line: &str) -> Option<MountPoint> {
        let mut elems = line.split_ascii_whitespace().take(4);
        if elems.clone().count() >= 4 {
            let volume = elems.next().unwrap();
            let path = elems.next().unwrap();
            let voltype = elems.next().unwrap();
            let options = elems.next().unwrap();
            return Some(Self::new(volume, path, voltype, options));
        }

        None
    }
}

pub type MountPoints = Vec<MountPoint>;
pub type Ifaces = Vec<IfaceDev>;
pub type Processes = Vec<Process>;

macro_rules! next_u64 {
    ($list:ident, $line:ident) => {
        $list
            .next()
            .unwrap()
            .parse::<u64>()
            .map_err(|e| Error::InvalidInputError($line.to_string(), e.to_string()))?
    };
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct IfaceDev {
    pub iface: String,

    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errs: u64,
    pub rx_drop: u64,
    pub rx_fifo: u64,
    pub rx_frame: u64,
    pub rx_compressed: u64,
    pub rx_multicast: u64,

    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errs: u64,
    pub tx_drop: u64,
    pub tx_fifo: u64,
    pub tx_frame: u64,
    pub tx_compressed: u64,
    pub tx_multicast: u64,
}
impl IfaceDev {
    pub(crate) fn from_line(line: &str) -> Result<IfaceDev, Error> {
        let mut elems = line.split_ascii_whitespace().take(17);
        if elems.clone().count() >= 17 {
            return Ok(IfaceDev {
                iface: elems.next().unwrap().trim_end_matches(':').to_string(),

                rx_bytes: next_u64!(elems, line),
                rx_packets: next_u64!(elems, line),
                rx_errs: next_u64!(elems, line),
                rx_drop: next_u64!(elems, line),
                rx_fifo: next_u64!(elems, line),
                rx_frame: next_u64!(elems, line),
                rx_compressed: next_u64!(elems, line),
                rx_multicast: next_u64!(elems, line),

                tx_bytes: next_u64!(elems, line),
                tx_packets: next_u64!(elems, line),
                tx_errs: next_u64!(elems, line),
                tx_drop: next_u64!(elems, line),
                tx_fifo: next_u64!(elems, line),
                tx_frame: next_u64!(elems, line),
                tx_compressed: next_u64!(elems, line),
                tx_multicast: next_u64!(elems, line),
            });
        }

        Err(Error::InvalidInputError(
            line.to_string(),
            "contains invalid input for IfaceDev".to_string(),
        ))
    }
}

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

        fn next<'l, T, I>(iter: &mut I, src: &str) -> Result<T, Error>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Display,
            I: Iterator<Item = &'l str>,
        {
            if let Some(s) = iter.next() {
                return s.parse::<T>().map_err(|e| {
                    Error::InvalidInputError(
                        src.to_string(),
                        format!("cannot parse '{}' as '{}' - '{}'", s, type_name::<T>(), e),
                    )
                });
            }

            Err(Error::InvalidInputError(
                src.to_string(),
                format!("there was no element of type {}", type_name::<T>()),
            ))
        }
        fn skip<I, T>(n: usize, iter: &mut I) -> &mut I
        where
            I: Iterator<Item = T>,
        {
            for _ in 0..n {
                iter.next();
            }
            iter
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_iface_from_net_dev_line() {
        let lo = IfaceDev {
            iface: "lo".to_string(),
            rx_bytes: 17776656,
            rx_packets: 127989,
            rx_errs: 0,
            rx_drop: 0,
            rx_fifo: 0,
            rx_frame: 0,
            rx_compressed: 0,
            rx_multicast: 0,

            tx_bytes: 17776656,
            tx_packets: 127989,
            tx_errs: 0,
            tx_drop: 0,
            tx_fifo: 0,
            tx_frame: 0,
            tx_compressed: 0,
            tx_multicast: 0,
        };
        let enp = IfaceDev {
            iface: "enp8s0".to_string(),
            rx_bytes: 482459368,
            rx_packets: 349468,
            rx_errs: 0,
            rx_drop: 0,
            rx_fifo: 0,
            rx_frame: 0,
            rx_compressed: 0,
            rx_multicast: 4785,

            tx_bytes: 16133415,
            tx_packets: 198549,
            tx_errs: 0,
            tx_drop: 0,
            tx_fifo: 0,
            tx_frame: 0,
            tx_compressed: 0,
            tx_multicast: 0,
        };
        let mut lines = NET_DEV.split('\n').skip(2);

        assert_eq!(Ok(lo), IfaceDev::from_line(lines.next().unwrap()));
        assert_eq!(Ok(enp), IfaceDev::from_line(lines.next().unwrap()))
    }

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
