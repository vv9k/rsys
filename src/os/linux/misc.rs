//! Other api
use crate::linux::SysFs;
use crate::Result;

#[cfg(feature = "serialize")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
pub struct MountPoints(Vec<MountPoint>);

#[derive(Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(Serialize, Deserialize))]
/// Represents a line of /proc/mounts
pub struct MountPoint {
    volume: String,
    path: String,
    voltype: String,
    options: String,
}

impl MountPoint {
    pub fn new(volume: &str, path: &str, voltype: &str, options: &str) -> MountPoint {
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
            let volume = elems.next()?;
            let path = elems.next()?;
            let voltype = elems.next()?;
            let options = elems.next()?;
            return Some(Self::new(volume, path, voltype, options));
        }

        None
    }
}

fn _mounts(out: &str) -> Result<MountPoints> {
    let mut mps = Vec::new();
    for line in out.split('\n') {
        if let Some(mp) = MountPoint::from_line(line) {
            mps.push(mp);
        }
    }
    Ok(MountPoints(mps))
}

/// Returns MountPoints read from /proc/mounts
pub fn mounts() -> Result<MountPoints> {
    _mounts(&SysFs::Proc.join("mounts").read()?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linux::mocks::MOUNTS;
    #[test]
    fn parses_mountpoints() {
        let expected = MountPoints(
              vec![
        MountPoint {
            volume: "proc".to_string(),
            path: "/proc".to_string(),
            voltype: "proc".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "sys".to_string(),
            path: "/sys".to_string(),
            voltype: "sysfs".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "dev".to_string(),
            path: "/dev".to_string(),
            voltype: "devtmpfs".to_string(),
            options: "rw,nosuid,relatime,size=8144744k,nr_inodes=2036186,mode=755,inode64".to_string(),
        },
        MountPoint {
            volume: "run".to_string(),
            path: "/run".to_string(),
            voltype: "tmpfs".to_string(),
            options: "rw,nosuid,nodev,relatime,mode=755,inode64".to_string(),
        },
        MountPoint {
            volume: "efivarfs".to_string(),
            path: "/sys/firmware/efi/efivars".to_string(),
            voltype: "efivarfs".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "/dev/mapper/vgroot-root".to_string(),
            path: "/".to_string(),
            voltype: "ext4".to_string(),
            options: "rw,relatime".to_string(),
        },
        MountPoint {
            volume: "securityfs".to_string(),
            path: "/sys/kernel/security".to_string(),
            voltype: "securityfs".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "tmpfs".to_string(),
            path: "/dev/shm".to_string(),
            voltype: "tmpfs".to_string(),
            options: "rw,nosuid,nodev,inode64".to_string(),
        },
        MountPoint {
            volume: "devpts".to_string(),
            path: "/dev/pts".to_string(),
            voltype: "devpts".to_string(),
            options: "rw,nosuid,noexec,relatime,gid=5,mode=620,ptmxmode=000".to_string(),
        },
        MountPoint {
            volume: "tmpfs".to_string(),
            path: "/sys/fs/cgroup".to_string(),
            voltype: "tmpfs".to_string(),
            options: "ro,nosuid,nodev,noexec,size=4096k,nr_inodes=1024,mode=755,inode64".to_string(),
        },
        MountPoint {
            volume: "cgroup2".to_string(),
            path: "/sys/fs/cgroup/unified".to_string(),
            voltype: "cgroup2".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,nsdelegate".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/systemd".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,xattr,name=systemd".to_string(),
        },
        MountPoint {
            volume: "pstore".to_string(),
            path: "/sys/fs/pstore".to_string(),
            voltype: "pstore".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "none".to_string(),
            path: "/sys/fs/bpf".to_string(),
            voltype: "bpf".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,mode=700".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/rdma".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,rdma".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/cpu,cpuacct".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,cpu,cpuacct".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/cpuset".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,cpuset".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/pids".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,pids".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/freezer".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,freezer".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/blkio".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,blkio".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/net_cls,net_prio".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,net_cls,net_prio".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/memory".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,memory".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/hugetlb".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,hugetlb".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/devices".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,devices".to_string(),
        },
        MountPoint {
            volume: "cgroup".to_string(),
            path: "/sys/fs/cgroup/perf_event".to_string(),
            voltype: "cgroup".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime,perf_event".to_string(),
        },
        MountPoint {
            volume: "systemd-1".to_string(),
            path: "/proc/sys/fs/binfmt_misc".to_string(),
            voltype: "autofs".to_string(),
            options: "rw,relatime,fd=30,pgrp=1,timeout=0,minproto=5,maxproto=5,direct,pipe_ino=12809".to_string(),
        },
        MountPoint {
            volume: "hugetlbfs".to_string(),
            path: "/dev/hugepages".to_string(),
            voltype: "hugetlbfs".to_string(),
            options: "rw,relatime,pagesize=2M".to_string(),
        },
        MountPoint {
            volume: "mqueue".to_string(),
            path: "/dev/mqueue".to_string(),
            voltype: "mqueue".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "debugfs".to_string(),
            path: "/sys/kernel/debug".to_string(),
            voltype: "debugfs".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "tracefs".to_string(),
            path: "/sys/kernel/tracing".to_string(),
            voltype: "tracefs".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "tmpfs".to_string(),
            path: "/tmp".to_string(),
            voltype: "tmpfs".to_string(),
            options: "rw,nosuid,nodev,nr_inodes=409600,inode64".to_string(),
        },
        MountPoint {
            volume: "configfs".to_string(),
            path: "/sys/kernel/config".to_string(),
            voltype: "configfs".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "fusectl".to_string(),
            path: "/sys/fs/fuse/connections".to_string(),
            voltype: "fusectl".to_string(),
            options: "rw,nosuid,nodev,noexec,relatime".to_string(),
        },
        MountPoint {
            volume: "/dev/mapper/vgroot-home".to_string(),
            path: "/home".to_string(),
            voltype: "ext4".to_string(),
            options: "rw,relatime".to_string(),
        },
        MountPoint {
            volume: "/dev/mapper/vgroot-var".to_string(),
            path: "/var".to_string(),
            voltype: "ext4".to_string(),
            options: "rw,relatime".to_string(),
        },
        MountPoint {
            volume: "/dev/nvme0n1p1".to_string(),
            path: "/boot".to_string(),
            voltype: "vfat".to_string(),
            options: "rw,relatime,fmask=0022,dmask=0022,codepage=437,iocharset=iso8859-1,shortname=mixed,utf8,errors=remount-ro".to_string(),
        },
        MountPoint {
            volume: "/dev/mapper/vgstor-rand".to_string(),
            path: "/mnt/rand".to_string(),
            voltype: "ext4".to_string(),
            options: "rw,relatime".to_string(),
        },
        MountPoint {
            volume: "/dev/mapper/vgstor-docs".to_string(),
            path: "/mnt/docs".to_string(),
            voltype: "ext4".to_string(),
            options: "rw,relatime".to_string(),
        },
        MountPoint {
            volume: "/dev/mapper/vgstor-media".to_string(),
            path: "/mnt/media".to_string(),
            voltype: "ext4".to_string(),
            options: "rw,relatime".to_string(),
        },
        MountPoint {
            volume: "tmpfs".to_string(),
            path: "/run/user/1000".to_string(),
            voltype: "tmpfs".to_string(),
            options: "rw,nosuid,nodev,relatime,size=1631388k,nr_inodes=407847,mode=700,uid=1000,gid=1000,inode64".to_string(),
        },
        MountPoint {
            volume: "gvfsd-fuse".to_string(),
            path: "/run/user/1000/gvfs".to_string(),
            voltype: "fuse.gvfsd-fuse".to_string(),
            options: "rw,nosuid,nodev,relatime,user_id=1000,group_id=1000".to_string(),
        },
    ]

        );

        assert_eq!(_mounts(MOUNTS).unwrap(), expected);
    }
}
