use super::*;
use crate::Result;

pub trait OsImplExt {
    // memory
    fn memory(&self) -> Result<Memory>;
    // ps
    fn stat_process(&self, pid: i32) -> Result<Process>;
    fn pids(&self) -> Result<Vec<i32>>;
    fn processes(&self) -> Result<Processes>;
    // storage
    fn stat_block_device(&self, name: &str) -> Result<StorageDevice>;
    fn stat_device_mapper(&self, name: &str) -> Result<DeviceMapper>;
    fn stat_scsi_cdrom(&self, name: &str) -> Result<ScsiCdrom>;
    fn stat_multiple_device_storage(&self, name: &str) -> Result<MultipleDeviceStorage>;
    // other
    fn kernel_version(&self) -> Result<String>;
    fn mounts(&self) -> Result<MountPoints>;
}

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
