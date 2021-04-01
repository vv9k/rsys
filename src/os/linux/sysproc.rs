#![allow(dead_code)]

use crate::{Error, Result};

use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

/// SysPath is an abstraction around procfs and sysfs. Allows for easy reading and parsing
/// of values in system paths.
#[derive(Clone, Debug)]
pub(crate) enum SysFs {
    Sys,
    Proc,
    Dev,
    Custom(PathBuf),
}

impl AsRef<str> for SysFs {
    fn as_ref(&self) -> &str {
        match &self {
            SysFs::Proc => "/proc",
            SysFs::Sys => "/sys",
            SysFs::Dev => "/dev",
            SysFs::Custom(s) => s.to_str().unwrap_or_default(),
        }
    }
}

impl From<SysFs> for SysPath {
    fn from(path: SysFs) -> Self {
        SysPath(PathBuf::from(path.as_ref()))
    }
}

impl From<SysPath> for PathBuf {
    fn from(path: SysPath) -> Self {
        path.0
    }
}

impl SysFs {
    pub(crate) fn join<P: AsRef<Path>>(self, path: P) -> SysPath {
        self.into_syspath().join(path)
    }

    pub(crate) fn into_syspath(self) -> SysPath {
        self.into()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SysPath(PathBuf);

impl SysPath {
    pub(crate) fn join<P: AsRef<Path>>(mut self, path: P) -> SysPath {
        self.0.push(path);
        self
    }

    /// Extends path with new elements returning a custom SysPath by cloning old one
    pub(crate) fn extend<P: AsRef<Path>>(&self, p: P) -> Self {
        let mut path = self.clone();
        path.0.push(p.as_ref());
        path
    }

    pub(crate) fn as_path(&self) -> &Path {
        self.0.as_path()
    }

    pub(crate) fn into_pathbuf(self) -> PathBuf {
        PathBuf::from(self)
    }

    /// Reads path to a string returning FileReadError on error
    pub(crate) fn read(&self) -> Result<String> {
        let path = self.as_path();
        fs::read_to_string(path).map_err(|e| Error::FileReadError(path.to_string_lossy().to_string(), e.to_string()))
    }

    /// Reads path and parses it as T otherwise returns FileReadError or InvalidInputError on error
    pub(crate) fn read_as<T: FromStr>(&self) -> Result<T>
    where
        <T as FromStr>::Err: Display,
    {
        let path = self.as_path();
        let data = fs::read_to_string(path)
            .map_err(|e| Error::FileReadError(path.to_string_lossy().to_string(), e.to_string()))?;

        T::from_str(data.trim()).map_err(|e| Error::InvalidInputError(data, e.to_string()))
    }

    /// Returns iterator over entries of this path
    pub(crate) fn read_dir(&self) -> Result<fs::ReadDir> {
        let path = self.as_path();
        fs::read_dir(path).map_err(|e| Error::FileReadError(path.to_string_lossy().to_string(), e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn correctly_joins_paths() {
        let mut path = SysPath(PathBuf::from("/proc/12/cpuset"));
        assert_eq!(path, SysFs::Proc.join("12").join("cpuset"));

        path = SysPath(PathBuf::from("/sys/block/sda"));
        assert_eq!(path, SysFs::Sys.join("block").join("sda"));

        path = SysPath(PathBuf::from("/dev/mapper/vgmain-root"));
        assert_eq!(path, SysFs::Dev.join("mapper").join("vgmain-root"));

        path = SysPath(PathBuf::from("/home/user/"));
        assert_eq!(path, SysFs::Custom(PathBuf::from("/home")).join("user"));
    }
}
