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
    cpu::{cores, cpu, cpu_clock, cpu_cores, logical_cores, processor, Core, Cores, Processor},
    mem::{memory, memory_free, memory_total, swap_free, swap_total, Memory},
    misc::{arch, domainname, hostname, kernel_version, mounts, uptime, MountPoint, MountPoints},
    net::{default_iface, ifaces, interfaces, ipv4, ipv6, mac, IfaceDev, Ifaces},
    os_impl_ext::OsImplExt,
    ps::{pids, processes, stat_process, Process, ProcessState, Processes},
    storage::{
        block_size, stat_block_device, stat_device_mapper, stat_multiple_device_storage, stat_scsi_cdrom,
        BlockStorageDeviceName, BlockStorageStat, DeviceMapper, DeviceMappers, MultipleDeviceStorage,
        MultipleDeviceStorages, Partition, Partitions, ScsiCdrom, ScsiCdroms, StorageDevice, StorageDevices,
    },
};

pub(crate) use sysproc::SysPath;

#[derive(Default, OsImpl)]
pub(crate) struct Linux {}
