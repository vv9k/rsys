use super::*;
use crate::Result;

/// Trait extending Rsys functionality with linux specific api
pub trait OsImplExt {
    // storage
    fn stat_block_device(&self, name: &str) -> Result<StorageDevice>;
    fn stat_device_mapper(&self, name: &str) -> Result<DeviceMapper>;
    fn stat_scsi_cdrom(&self, name: &str) -> Result<ScsiCdrom>;
    fn stat_multiple_device_storage(&self, name: &str) -> Result<MultipleDeviceStorage>;
    fn block_size(&self, name: &str) -> Result<i64>;
    // memory
    fn memory(&self) -> Result<Memory>;
    // ps
    fn stat_process(&self, pid: i32) -> Result<Process>;
    fn pids(&self) -> Result<Vec<i32>>;
    fn processes(&self) -> Result<Processes>;
    // other
    fn kernel_version(&self) -> Result<String>;
    fn mounts(&self) -> Result<MountPoints>;
    // cpu
    fn cores(&self) -> Result<Cores>;
    fn processor(&self) -> Result<Processor>;
}

impl OsImplExt for Linux {
    // storage

    /// Parses a StorageDevice object from system. If the provided name
    /// doesn't start with `sd` returns an error.
    fn stat_block_device(&self, name: &str) -> Result<StorageDevice> {
        stat_block_device(name)
    }
    /// Parses a DeviceMapper object from system. If the provided name
    /// doesn't start with `dm` returns an error.
    fn stat_device_mapper(&self, name: &str) -> Result<DeviceMapper> {
        stat_device_mapper(name)
    }
    /// Parses a ScsiCdrom object from system. If the provided name
    /// doesn't start with `sr` returns an error.
    fn stat_scsi_cdrom(&self, name: &str) -> Result<ScsiCdrom> {
        stat_scsi_cdrom(name)
    }
    /// Parses a MultipleDeviceStorage object from system. If the provided name
    /// doesn't start with `md` returns an error.
    fn stat_multiple_device_storage(&self, name: &str) -> Result<MultipleDeviceStorage> {
        stat_multiple_device_storage(name)
    }
    /// Returns block size of device in bytes
    /// device argument must be a path to block device file descriptor
    fn block_size(&self, name: &str) -> Result<i64> {
        block_size(name)
    }

    // mem

    /// Returns detailed information about memory
    fn memory(&self) -> Result<Memory> {
        memory()
    }

    // ps

    /// Returns detailed Process information parsed from /proc/[pid]/stat
    fn stat_process(&self, pid: i32) -> Result<Process> {
        stat_process(pid)
    }
    /// Returns a list of pids read from /proc
    fn pids(&self) -> Result<Vec<i32>> {
        pids()
    }
    /// Returns all processes currently seen in /proc parsed as Processes
    fn processes(&self) -> Result<Processes> {
        processes()
    }

    // other

    /// Returns kernel version of host os.
    fn kernel_version(&self) -> Result<String> {
        kernel_version()
    }
    /// Returns MountPoints read from /proc/mounts
    fn mounts(&self) -> Result<MountPoints> {
        mounts()
    }

    // cpu

    /// Returns virtual Cores of host cpu
    fn cores(&self) -> Result<Cores> {
        cores()
    }
    /// Returns a Processor object containing gathered information about host cpu
    fn processor(&self) -> Result<Processor> {
        processor()
    }
}
