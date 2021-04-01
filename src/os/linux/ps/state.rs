#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum ProcessState {
    Running,
    Sleeping,
    Waiting,
    Zombie,
    Stopped,
    TracingStop,
    Dead,
    Wakekill,
    Waking,
    Parked,
    Idle,
    Unknown,
}

impl From<&str> for ProcessState {
    fn from(s: &str) -> Self {
        use self::ProcessState::*;
        match s.chars().next() {
            Some('R') => Running,
            Some('S') => Sleeping,
            Some('D') => Waiting,
            Some('Z') => Zombie,
            Some('T') => Stopped,
            Some('t') => TracingStop,
            Some('X') | Some('x') => Dead,
            Some('K') => Wakekill,
            Some('W') => Waking,
            Some('P') => Parked,
            Some('I') => Idle,
            _ => Unknown,
        }
    }
}
