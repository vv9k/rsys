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
