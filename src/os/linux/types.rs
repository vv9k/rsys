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

macro_rules! next_u64 {
    ($list:ident) => {
        $list
            .next()
            .unwrap()
            .parse::<u64>()
            .map_err(|e| Error::InvalidInputError(e.to_string()))?
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

                rx_bytes: next_u64!(elems),
                rx_packets: next_u64!(elems),
                rx_errs: next_u64!(elems),
                rx_drop: next_u64!(elems),
                rx_fifo: next_u64!(elems),
                rx_frame: next_u64!(elems),
                rx_compressed: next_u64!(elems),
                rx_multicast: next_u64!(elems),

                tx_bytes: next_u64!(elems),
                tx_packets: next_u64!(elems),
                tx_errs: next_u64!(elems),
                tx_drop: next_u64!(elems),
                tx_fifo: next_u64!(elems),
                tx_frame: next_u64!(elems),
                tx_compressed: next_u64!(elems),
                tx_multicast: next_u64!(elems),
            });
        }

        Err(Error::InvalidInputError(
            "Line contains invalid proc/net/dev output".to_string(),
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
    pid: i32,
    name: String,
    state: ProcessState,
    ppid: i32,
    pgrp: i32,
    session: i32,
    tty_nr: i32,
    utime: u64,
    stime: u64,
    cutime: i64,
    cstime: i64,
    priority: i32,
    nice: i32,
    num_threads: i32,
    itrealvalue: i32,
    starttime: u64,
    vsize: u32,
    rss: i32,
    rsslim: u64,
    nswap: u32,
    cnswap: u32,
    guest_time: u32,
    cguest_time: u32,
}

impl Process {
    pub(crate) fn from_stat(stat: &str) -> Result<Process, Error> {
        let mut elems = stat.split_ascii_whitespace();

        fn next<'l, T, I>(iter: &mut I) -> Result<T, Error>
        where
            T: std::str::FromStr,
            T::Err: std::fmt::Display,
            I: Iterator<Item = &'l str>,
        {
            if let Some(s) = iter.next() {
                return s.parse::<T>().map_err(|e| Error::InvalidInputError(e.to_string()));
            }

            Err(Error::InvalidInputError(format!(
                "element is not of type {}",
                type_name::<T>()
            )))
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
            pid: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            name: next::<String, SplitAsciiWhitespace>(&mut elems)?,
            state: ProcessState::from(next::<String, SplitAsciiWhitespace>(&mut elems)?.as_str()),
            ppid: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            pgrp: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            session: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            tty_nr: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            utime: next::<u64, SplitAsciiWhitespace>(skip(6, &mut elems))?,
            stime: next::<u64, SplitAsciiWhitespace>(&mut elems)?,
            cutime: next::<i64, SplitAsciiWhitespace>(&mut elems)?,
            cstime: next::<i64, SplitAsciiWhitespace>(&mut elems)?,
            priority: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            nice: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            num_threads: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            itrealvalue: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            starttime: next::<u64, SplitAsciiWhitespace>(&mut elems)?,
            vsize: next::<u32, SplitAsciiWhitespace>(&mut elems)?,
            rss: next::<i32, SplitAsciiWhitespace>(&mut elems)?,
            rsslim: next::<u64, SplitAsciiWhitespace>(&mut elems)?,
            nswap: next::<u32, SplitAsciiWhitespace>(skip(10, &mut elems))?,
            cnswap: next::<u32, SplitAsciiWhitespace>(&mut elems)?,
            guest_time: next::<u32, SplitAsciiWhitespace>(skip(5, &mut elems))?,
            cguest_time: next::<u32, SplitAsciiWhitespace>(&mut elems)?,
        })
    }
}
