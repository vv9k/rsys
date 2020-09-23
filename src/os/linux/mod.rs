//! Linux specific api
#![cfg(target_os = "linux")]

#[cfg(test)]
pub(crate) mod mocks;

mod cpu;
mod mem;
mod misc;
mod net;
mod os_impl_ext;
mod ps;
mod storage;
mod sysproc;

use super::{run, OsImpl};
use crate::Result;

pub use {
    cpu::{cpu, cpu_clock, cpu_cores, logical_cores},
    mem::{memory, memory_free, memory_total, swap_free, swap_total, Memory},
    misc::{arch, domainname, hostname, kernel_version, mounts, uptime, MountPoint, MountPoints},
    net::{default_iface, ifaces, interfaces, ipv4, ipv6, mac, IfaceDev, Ifaces},
    os_impl_ext::OsImplExt,
    ps::{pids, processes, stat_process, Process, ProcessState, Processes},
    storage::{
        stat_block_device, stat_device_mapper, stat_multiple_device_storage, stat_scsi_cdrom, BlockStorageStat,
        DeviceMapper, DeviceMappers, MultipleDeviceStorage, MultipleDeviceStorages, Partition, Partitions, ScsiCdrom,
        ScsiCdroms, StorageDevice, StorageDevices,
    },
};

pub(crate) use sysproc::SysPath;

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}

impl OsImplExt for Linux {
    // storage
    fn stat_block_device(&self, name: &str) -> Result<StorageDevice> {
        stat_block_device(name)
    }
    fn stat_device_mapper(&self, name: &str) -> Result<DeviceMapper> {
        stat_device_mapper(name)
    }
    fn stat_scsi_cdrom(&self, name: &str) -> Result<ScsiCdrom> {
        stat_scsi_cdrom(name)
    }
    fn stat_multiple_device_storage(&self, name: &str) -> Result<MultipleDeviceStorage> {
        stat_multiple_device_storage(name)
    }
    // mem
    fn memory(&self) -> Result<Memory> {
        memory()
    }
    // ps
    fn stat_process(&self, pid: i32) -> Result<Process> {
        stat_process(pid)
    }
    fn pids(&self) -> Result<Vec<i32>> {
        pids()
    }
    fn processes(&self) -> Result<Processes> {
        processes()
    }
    // other
    fn kernel_version(&self) -> Result<String> {
        kernel_version()
    }
    fn mounts(&self) -> Result<MountPoints> {
        mounts()
    }
}
