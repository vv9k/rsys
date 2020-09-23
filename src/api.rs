#[cfg(target_os = "linux")]
use crate::linux::{
    DeviceMapper, Linux, Memory, MountPoints, MultipleDeviceStorage, Process, Processes, ScsiCdrom, StorageDevice,
};
#[cfg(target_os = "macos")]
use crate::macos::Macos;
#[cfg(target_os = "windows")]
use crate::windows::Windows;

use crate::{
    os::{OsImpl, OsImplExt},
    Result,
};
use std::boxed::Box;
use std::env;

/// Main interface that allows for os-agnostic api.
pub struct Rsys(Box<dyn OsImpl>, Box<dyn OsImplExt>);

impl Rsys {
    #[cfg(target_os = "linux")]
    /// Creates a new instance of Rsys
    pub fn new() -> Self {
        Self(
            Box::new(Linux::new()) as Box<dyn OsImpl>,
            Box::new(Linux::new()) as Box<dyn OsImplExt>,
        )
    }

    #[cfg(target_os = "macos")]
    /// Creates a new instance of Rsys
    pub fn new() -> Self {
        Self(Box::new(Macos::new()) as Box<dyn OsImpl>)
    }

    #[cfg(target_os = "windows")]
    /// Creates a new instance of Rsys
    pub fn new() -> Self {
        Self(Box::new(Windows::new()) as Box<dyn OsImpl>)
    }

    /// Returns a hostname.
    ///   * **linux**
    ///     * by reading `/proc/sys/kernel/hostname`
    ///   * **macos**
    ///     * by calling `sysctl("kern.hostname")`
    ///   * **windows**
    ///     * by calling win32 api `GetComputerNameA`
    pub fn hostname(&self) -> Result<String> {
        self.0.hostname()
    }

    /// Returns time since boot in seconds.
    ///   * **linux**
    ///     * by reading `/proc/uptime`
    ///   * **macos**
    ///     * by calling `sysctl("kern.boottime")`
    ///   * **windows**
    ///     * by calling win32 api `GetTickCount64`
    pub fn uptime(&self) -> Result<u64> {
        self.0.uptime()
    }

    /// Returns operating system. Reexported `env::consts::OS` for convenience.
    pub fn os(&self) -> String {
        env::consts::OS.to_string()
    }

    /// Returns cpu architecture.
    ///   * **linux** and **macos**
    ///     * by calling `uname -m`
    ///   * **windows**
    ///     * by calling win32 api `GetSystemInfo`
    pub fn arch(&self) -> Result<String> {
        self.0.arch()
    }
    /// Returns a cpu model name.
    ///   * **linux**
    ///     * by reading `/proc/cpuinfo`
    ///   * **macos**
    ///     * by calling `sysctl("machdep.cpu.brand_string")`
    ///   ...
    pub fn cpu(&self) -> Result<String> {
        self.0.cpu()
    }

    /// Returns clock speed of cpu.
    ///   * **linux**
    ///     * by reading `/proc/cpuinfo`
    ///   * **macos**
    ///     * by calling `sysctl("hw.cpufrequency")`
    ///   ...
    pub fn cpu_clock(&self) -> Result<f32> {
        self.0.cpu_clock()
    }

    /// Returns cpu cores.
    ///   * **linux**
    ///     * by reading `/proc/cpuinfo`
    ///   * **macos**
    ///     * by calling `sysctl("hw.physicalcpu")`
    ///   * **windows**
    ///     * by determining if cpu is hyperthreaded and calculating from logical cores.
    pub fn cpu_cores(&self) -> Result<u16> {
        self.0.cpu_cores()
    }

    /// Returns logical cpu cores.
    ///   * **linux**
    ///     * by reading `/proc/cpuinfo`
    ///   * **macos**
    ///     * by calling `sysctl("hw.logicalcpu")`
    ///   * **windows**
    ///     * by calling win32 api `GetSystemInfo`
    pub fn logical_cores(&self) -> Result<u16> {
        self.0.logical_cores()
    }

    /// Returns total ram memory.
    ///   * **linux**
    ///     * by reading `/proc/meminfo`
    ///   * **macos**
    ///     * by calling `sysctl("hw.memsize")`
    ///   * **windows**
    ///     * by calling win32 api `GlobalMemoryStatusEx`
    pub fn memory_total(&self) -> Result<usize> {
        self.0.memory_total()
    }

    /// Returns free ram memory.
    ///   * **linux**
    ///     * by reading `/proc/meminfo`
    ///   ...
    pub fn memory_free(&self) -> Result<usize> {
        self.0.memory_free()
    }

    /// Returns total swap size.
    ///   * **linux**
    ///     * by reading `/proc/meminfo`
    ///   * **macos**
    ///     * by calling `sysctl("hw.swapusage")`
    ///   * **windows**
    ///     * by calling win32 api `GlobalMemoryStatusEx`
    pub fn swap_total(&self) -> Result<usize> {
        self.0.swap_total()
    }

    /// Returns free swap size.
    ///   * **linux**
    ///     * by reading `/proc/meminfo`
    ///   * **macos**
    ///     * by calling `sysctl("hw.swapusage")`
    ///   * **windows**
    ///     * by calling win32 api `GlobalMemoryStatusEx`
    pub fn swap_free(&self) -> Result<usize> {
        self.0.swap_free()
    }

    /// Returns a default internet interface.
    ///   * **linux**
    ///     * by calling `route` and determining default route
    ///   * **macos**
    ///     * by calling `route get default`
    ///   ...
    pub fn default_iface(&self) -> Result<String> {
        self.0.default_iface()
    }

    /// Returns IP address version 4 of interface iface.
    ///   * **linux**
    ///     * by calling `ip address show <iface>`
    ///   * **macos**
    ///     * by calling `ipconfig getifaddr <iface>`
    ///   ...
    pub fn ipv4(&self, iface: &str) -> Result<String> {
        self.0.ipv4(iface)
    }

    /// Returns IP address version 6 of interface iface.
    pub fn ipv6(&self, iface: &str) -> Result<String> {
        self.0.ipv6(iface)
    }

    /// Returns a MAC address of interface iface.
    ///   * **linux**
    ///     * by calling `ip address show <iface>`
    ///   ...
    pub fn mac(&self, iface: &str) -> Result<String> {
        self.0.mac(iface)
    }

    /// Returns a vector of names of network interfaces that are available.
    ///   * **linux**
    ///     * by calling `ip address show`
    ///   ...
    pub fn interfaces(&self) -> Result<Vec<String>> {
        self.0.interfaces()
    }

    /// Returns a domain name.
    ///   * **linux**
    ///     * by reading `/proc/sys/kernel/domainname`
    ///   * **windows**
    ///     * by calling win32 api `NetWkstaGetInfo`
    ///   ...
    pub fn domainname(&self) -> Result<String> {
        self.0.domainname()
    }

    #[cfg(target_os = "linux")]
    pub fn stat_block_device(&self, name: &str) -> Result<StorageDevice> {
        self.1.stat_block_device(name)
    }

    #[cfg(target_os = "linux")]
    pub fn stat_scsi_cdrom(&self, name: &str) -> Result<ScsiCdrom> {
        self.1.stat_scsi_cdrom(name)
    }

    #[cfg(target_os = "linux")]
    pub fn stat_device_mapper(&self, name: &str) -> Result<DeviceMapper> {
        self.1.stat_device_mapper(name)
    }

    #[cfg(target_os = "linux")]
    pub fn stat_multiple_device_storage(&self, name: &str) -> Result<MultipleDeviceStorage> {
        self.1.stat_multiple_device_storage(name)
    }

    #[cfg(target_os = "linux")]
    pub fn memory(&self) -> Result<Memory> {
        self.1.memory()
    }

    #[cfg(target_os = "linux")]
    pub fn stat_process(&self, pid: i32) -> Result<Process> {
        self.1.stat_process(pid)
    }
    #[cfg(target_os = "linux")]
    pub fn pids(&self) -> Result<Vec<i32>> {
        self.1.pids()
    }
    #[cfg(target_os = "linux")]
    pub fn processes(&self) -> Result<Processes> {
        self.1.processes()
    }
    #[cfg(target_os = "linux")]
    pub fn kernel_version(&self) -> Result<String> {
        self.1.kernel_version()
    }
    #[cfg(target_os = "linux")]
    pub fn mounts(&self) -> Result<MountPoints> {
        self.1.mounts()
    }
}
