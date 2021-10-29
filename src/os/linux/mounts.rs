use crate::linux::SysFs;
use crate::Result;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};
use std::convert::AsRef;

#[derive(Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct MountPoints(Vec<MountPoint>);

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a mountpoint parsed from line of /proc/mounts
pub struct MountPoint {
    pub volume: String,
    pub path: String,
    pub voltype: String,
    pub mount_mode: MountMode,
    options: Vec<String>,
}

/// Represents an option `ro` or `rw` deciding wether the mountpoint is mounter with read only or
/// read and write permissions.
#[derive(Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub enum MountMode {
    ReadWrite,
    ReadOnly,
}

impl MountMode {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "ro" => Some(MountMode::ReadOnly),
            "rw" => Some(MountMode::ReadWrite),
            _ => None,
        }
    }
}

impl AsRef<str> for MountMode {
    fn as_ref(&self) -> &str {
        match &self {
            MountMode::ReadOnly => "ro",
            MountMode::ReadWrite => "rw",
        }
    }
}

impl MountPoint {
    pub fn options(&self) -> &[String] {
        &self.options
    }

    pub(crate) fn new(volume: &str, path: &str, voltype: &str, options: &str) -> MountPoint {
        let options: Vec<String> = options.split(',').map(str::to_string).collect();
        let mut mount_mode = MountMode::ReadOnly;

        for opt in options.iter() {
            if let Some(mm) = MountMode::from_str(opt) {
                mount_mode = mm;
            }
        }

        MountPoint {
            volume: volume.to_string(),
            path: path.to_string(),
            voltype: voltype.to_string(),
            mount_mode,
            options,
        }
    }

    pub(crate) fn from_line(line: &str) -> Option<MountPoint> {
        let mut elems = line.split_ascii_whitespace().take(4);
        if elems.clone().count() >= 4 {
            let volume = elems.next()?;
            let path = elems.next()?;
            let voltype = elems.next()?;
            let options = elems.next()?;
            return Some(Self::new(volume, path, voltype, options));
        }

        None
    }
}

/// Returns `MountPoints` read from `/proc/mounts`
pub fn mounts() -> Result<MountPoints> {
    Ok(_mounts(&SysFs::Proc.join("mounts").read()?))
}

fn _mounts(out: &str) -> MountPoints {
    let mut mps = Vec::new();
    for line in out.split('\n') {
        if let Some(mp) = MountPoint::from_line(line) {
            mps.push(mp);
        }
    }
    MountPoints(mps)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linux::mocks::MOUNTS;

    #[test]
    fn parses_mountpoints() {
        let expected = MountPoints(vec![
            MountPoint {
                volume: "proc".to_string(),
                path: "/proc".to_string(),
                voltype: "proc".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "sys".to_string(),
                path: "/sys".to_string(),
                voltype: "sysfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "dev".to_string(),
                path: "/dev".to_string(),
                voltype: "devtmpfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "relatime".to_string(),
                    "size=8144744k".to_string(),
                    "nr_inodes=2036186".to_string(),
                    "mode=755".to_string(),
                    "inode64".to_string(),
                ],
            },
            MountPoint {
                volume: "run".to_string(),
                path: "/run".to_string(),
                voltype: "tmpfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "relatime".to_string(),
                    "mode=755".to_string(),
                    "inode64".to_string(),
                ],
            },
            MountPoint {
                volume: "efivarfs".to_string(),
                path: "/sys/firmware/efi/efivars".to_string(),
                voltype: "efivarfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "/dev/mapper/vgroot-root".to_string(),
                path: "/".to_string(),
                voltype: "ext4".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec!["rw".to_string(), "relatime".to_string()],
            },
            MountPoint {
                volume: "securityfs".to_string(),
                path: "/sys/kernel/security".to_string(),
                voltype: "securityfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "tmpfs".to_string(),
                path: "/dev/shm".to_string(),
                voltype: "tmpfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "inode64".to_string(),
                ],
            },
            MountPoint {
                volume: "devpts".to_string(),
                path: "/dev/pts".to_string(),
                voltype: "devpts".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "gid=5".to_string(),
                    "mode=620".to_string(),
                    "ptmxmode=000".to_string(),
                ],
            },
            MountPoint {
                volume: "tmpfs".to_string(),
                path: "/sys/fs/cgroup".to_string(),
                voltype: "tmpfs".to_string(),
                mount_mode: MountMode::ReadOnly,
                options: vec![
                    "ro".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "size=4096k".to_string(),
                    "nr_inodes=1024".to_string(),
                    "mode=755".to_string(),
                    "inode64".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup2".to_string(),
                path: "/sys/fs/cgroup/unified".to_string(),
                voltype: "cgroup2".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "nsdelegate".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/systemd".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "xattr".to_string(),
                    "name=systemd".to_string(),
                ],
            },
            MountPoint {
                volume: "pstore".to_string(),
                path: "/sys/fs/pstore".to_string(),
                voltype: "pstore".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "none".to_string(),
                path: "/sys/fs/bpf".to_string(),
                voltype: "bpf".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "mode=700".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/rdma".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "rdma".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/cpu,cpuacct".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "cpu".to_string(),
                    "cpuacct".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/cpuset".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "cpuset".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/pids".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "pids".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/freezer".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "freezer".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/blkio".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "blkio".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/net_cls,net_prio".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "net_cls".to_string(),
                    "net_prio".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/memory".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "memory".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/hugetlb".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "hugetlb".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/devices".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "devices".to_string(),
                ],
            },
            MountPoint {
                volume: "cgroup".to_string(),
                path: "/sys/fs/cgroup/perf_event".to_string(),
                voltype: "cgroup".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                    "perf_event".to_string(),
                ],
            },
            MountPoint {
                volume: "systemd-1".to_string(),
                path: "/proc/sys/fs/binfmt_misc".to_string(),
                voltype: "autofs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "relatime".to_string(),
                    "fd=30".to_string(),
                    "pgrp=1".to_string(),
                    "timeout=0".to_string(),
                    "minproto=5".to_string(),
                    "maxproto=5".to_string(),
                    "direct".to_string(),
                    "pipe_ino=12809".to_string(),
                ],
            },
            MountPoint {
                volume: "hugetlbfs".to_string(),
                path: "/dev/hugepages".to_string(),
                voltype: "hugetlbfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec!["rw".to_string(), "relatime".to_string(), "pagesize=2M".to_string()],
            },
            MountPoint {
                volume: "mqueue".to_string(),
                path: "/dev/mqueue".to_string(),
                voltype: "mqueue".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "debugfs".to_string(),
                path: "/sys/kernel/debug".to_string(),
                voltype: "debugfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "tracefs".to_string(),
                path: "/sys/kernel/tracing".to_string(),
                voltype: "tracefs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "tmpfs".to_string(),
                path: "/tmp".to_string(),
                voltype: "tmpfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "nr_inodes=409600".to_string(),
                    "inode64".to_string(),
                ],
            },
            MountPoint {
                volume: "configfs".to_string(),
                path: "/sys/kernel/config".to_string(),
                voltype: "configfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "fusectl".to_string(),
                path: "/sys/fs/fuse/connections".to_string(),
                voltype: "fusectl".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "noexec".to_string(),
                    "relatime".to_string(),
                ],
            },
            MountPoint {
                volume: "/dev/mapper/vgroot-home".to_string(),
                path: "/home".to_string(),
                voltype: "ext4".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec!["rw".to_string(), "relatime".to_string()],
            },
            MountPoint {
                volume: "/dev/mapper/vgroot-var".to_string(),
                path: "/var".to_string(),
                voltype: "ext4".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec!["rw".to_string(), "relatime".to_string()],
            },
            MountPoint {
                volume: "/dev/nvme0n1p1".to_string(),
                path: "/boot".to_string(),
                voltype: "vfat".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "relatime".to_string(),
                    "fmask=0022".to_string(),
                    "dmask=0022".to_string(),
                    "codepage=437".to_string(),
                    "iocharset=iso8859-1".to_string(),
                    "shortname=mixed".to_string(),
                    "utf8".to_string(),
                    "errors=remount-ro".to_string(),
                ],
            },
            MountPoint {
                volume: "/dev/mapper/vgstor-rand".to_string(),
                path: "/mnt/rand".to_string(),
                voltype: "ext4".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec!["rw".to_string(), "relatime".to_string()],
            },
            MountPoint {
                volume: "/dev/mapper/vgstor-docs".to_string(),
                path: "/mnt/docs".to_string(),
                voltype: "ext4".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec!["rw".to_string(), "relatime".to_string()],
            },
            MountPoint {
                volume: "/dev/mapper/vgstor-media".to_string(),
                path: "/mnt/media".to_string(),
                voltype: "ext4".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec!["rw".to_string(), "relatime".to_string()],
            },
            MountPoint {
                volume: "tmpfs".to_string(),
                path: "/run/user/1000".to_string(),
                voltype: "tmpfs".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "relatime".to_string(),
                    "size=1631388k".to_string(),
                    "nr_inodes=407847".to_string(),
                    "mode=700".to_string(),
                    "uid=1000".to_string(),
                    "gid=1000".to_string(),
                    "inode64".to_string(),
                ],
            },
            MountPoint {
                volume: "gvfsd-fuse".to_string(),
                path: "/run/user/1000/gvfs".to_string(),
                voltype: "fuse.gvfsd-fuse".to_string(),
                mount_mode: MountMode::ReadWrite,
                options: vec![
                    "rw".to_string(),
                    "nosuid".to_string(),
                    "nodev".to_string(),
                    "relatime".to_string(),
                    "user_id=1000".to_string(),
                    "group_id=1000".to_string(),
                ],
            },
        ]);

        assert_eq!(_mounts(MOUNTS), expected);
    }
}
