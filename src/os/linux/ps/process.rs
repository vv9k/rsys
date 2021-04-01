use crate::linux::ps::{cmdline, ProcessStat};
use crate::linux::{SysFs, SysPath};
use crate::Result;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

pub type Processes = Vec<Process>;

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Task {
    pub cmdline: String,
    pub stat: ProcessStat,
}
impl Task {
    pub(crate) fn from_sys_path(path: &SysPath) -> Result<Task> {
        Ok(Task {
            cmdline: cmdline(&path)?,
            stat: ProcessStat::from_sys_path(&path)?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct Process {
    pub cmdline: String,
    pub stat: ProcessStat,
}
impl Process {
    pub fn new(pid: i32) -> Result<Process> {
        let p = SysFs::Proc.join(pid.to_string());
        Ok(Process {
            cmdline: cmdline(&p)?,
            stat: ProcessStat::from_sys_path(&p)?,
        })
    }

    pub fn tasks(&self) -> Result<Vec<Task>> {
        let mut tasks = Vec::new();
        for entry in SysFs::Proc.join(self.stat.pid.to_string()).join("task").read_dir()? {
            if let Ok(entry) = entry {
                tasks.push(Task::from_sys_path(&SysFs::Custom(entry.path()).to_syspath())?);
            }
        }
        Ok(tasks)
    }
}
