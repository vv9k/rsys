//! All about processes

mod process;
mod stat;
mod state;

pub use process::*;
pub use stat::*;
pub use state::*;

use crate::linux::{SysFs, SysPath};
use crate::{Error, Result};

use std::fs;

fn cmdline(path: SysPath) -> Result<String> {
    path.join("cmdline")
        .read()
        .map(|s| s.trim_end_matches('\x00').replace('\x00', " "))
}

/// Returns detailed Process information parsed from /proc/[pid]/stat
pub fn stat_process(pid: i32) -> Result<ProcessStat> {
    ProcessStat::from_stat(&SysFs::Proc.join(pid.to_string()).join("stat").read()?)
}

/// Returns a list of pids read from /proc
pub fn pids() -> Result<Vec<i32>> {
    let path = SysFs::Proc.to_syspath().path();
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
pub fn processes() -> Result<Processes> {
    let mut ps = Vec::new();
    for pid in pids()? {
        ps.push(Process::new(pid)?);
    }

    Ok(ps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io};
    #[test]
    fn parses_cmdline() -> io::Result<()> {
        let line = "/usr/lib/firefox/firefox\x00-contentproc\x00-childID\x001\x00-isForBrowser\x00-prefsLen\x001\x00-prefMapSize\x00234803\x00-parentBuildID\x0020201001181215\x00-appdir\x00/usr/lib/firefox/browser\x006732\x00true\x00tab\x00";

        let dir = tempfile::tempdir()?;
        fs::write(dir.path().join("cmdline"), line)?;

        let after = "/usr/lib/firefox/firefox -contentproc -childID 1 -isForBrowser -prefsLen 1 -prefMapSize 234803 -parentBuildID 20201001181215 -appdir /usr/lib/firefox/browser 6732 true tab".to_string();

        assert_eq!(Ok(after), cmdline(SysFs::Custom(dir.path().to_owned()).to_syspath()));

        dir.close()
    }
}
