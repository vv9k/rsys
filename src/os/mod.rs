#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

use super::error::RsysError as Error;
use std::env;
use std::process::Command;
use std::str;

#[cfg(target_os = "linux")]
use linux::*;
#[cfg(target_os = "macos")]
use macos::*;
#[cfg(target_os = "windows")]
use windows::*;

// Internal function for mapping errors on command execution
fn run(cmd: &mut Command) -> Result<String, Error> {
    match cmd.output() {
        Ok(out) => match str::from_utf8(&out.stdout) {
            Ok(out) => Ok(out.trim_end_matches('\n').to_string()),
            Err(e) => Err(Error::CommandParseError(e.to_string())),
        },
        Err(e) => Err(Error::CommandRunError(e.to_string())),
    }
}

/// Returns a hostname.
///   * **linux**
///     * by reading `/proc/sys/kernel/hostname`
///   * **macos**
///     * by calling `sysctl("kern.hostname")`
///   * **windows**
///     * by calling win32 api `GetComputerNameA`
pub fn hostname() -> Result<String, Error> {
    _hostname()
}

/// Returns time since boot in seconds.
///   * **linux**
///     * by reading `/proc/uptime`
///   * **macos**
///     * by calling `sysctl("kern.boottime")`
///   * **windows**
///     * by calling win32 api `GetTickCount64`
pub fn uptime() -> Result<u64, Error> {
    _uptime()
}

/// Returns operating system. Reexported `env::consts::OS` for convenience.
pub fn os() -> String {
    env::consts::OS.to_string()
}

/// Returns cpu architecture.
///   * **linux** and **macos**
///     * by calling `uname -m`
///   * **windows**
///     * by calling win32 api `GetSystemInfo`
pub fn arch() -> Result<String, Error> {
    _arch()
}
/// Returns a cpu model name.
///   * **linux**
///     * by reading `/proc/cpuinfo`
///   * **macos**
///     * by calling `sysctl("machdep.cpu.brand_string")`
///   ...
pub fn cpu() -> Result<String, Error> {
    _cpu()
}

/// Returns clock speed of cpu.
///   * **linux**
///     * by reading `/proc/cpuinfo`
///   * **macos**
///     * by calling `sysctl("hw.cpufrequency")`
///   ...
pub fn cpu_clock() -> Result<f32, Error> {
    _cpu_clock()
}

/// Returns cpu cores.
///   * **linux**
///     * by reading `/proc/cpuinfo`
///   * **macos**
///     * by calling `sysctl("hw.physicalcpu")`
///   * **windows**
///     * by determining if cpu is hyperthreaded and calculating from logical cores.
pub fn cpu_cores() -> Result<u16, Error> {
    _cpu_cores()
}

/// Returns logical cpu cores.
///   * **linux**
///     * by reading `/proc/cpuinfo`
///   * **macos**
///     * by calling `sysctl("hw.logicalcpu")`
///   * **windows**
///     * by calling win32 api `GetSystemInfo`
pub fn logical_cores() -> Result<u16, Error> {
    _logical_cores()
}

/// Returns total ram memory.
///   * **linux**
///     * by reading `/proc/meminfo`
///   * **macos**
///     * by calling `sysctl("hw.memsize")`
///   * **windows**
///     * by calling win32 api `GlobalMemoryStatusEx`
pub fn memory() -> Result<usize, Error> {
    _memory()
}

/// Returns total swap size.
///   * **linux**
///     * by reading `/proc/meminfo`
///   * **macos**
///     * by calling `sysctl("hw.swapusage")`
///   * **windows**
///     * by calling win32 api `GlobalMemoryStatusEx`
pub fn swap() -> Result<usize, Error> {
    _swap()
}

/// Returns a default internet interface.
///   * **linux**
///     * by calling `route` and determining default route
///   * **macos**
///     * by calling `route get default`
///   ...
pub fn default_iface() -> Result<String, Error> {
    _default_iface()
}

/// Returns IP address version 4 of interface iface.
///   * **linux**
///     * by calling `ip address show <iface>`
///   * **macos**
///     * by calling `ipconfig getifaddr <iface>`
///   ...
pub fn ipv4(iface: &str) -> Result<String, Error> {
    _ipv4(iface)
}

/// Returns IP address version 6 of interface iface.
pub fn ipv6(iface: &str) -> Result<String, Error> {
    _ipv6(iface)
}

/// Returns a MAC address of interface iface.
///   * **linux**
///     * by calling `ip address show <iface>`
///   ...
pub fn mac(iface: &str) -> Result<String, Error> {
    _mac(iface)
}

/// Returns a vector of names of network interfaces that are available.
///   * **linux**
///     * by calling `ip address show`
///   ...
pub fn interfaces() -> Result<Vec<String>, Error> {
    _interfaces()
}

/// Returns a domain name.
///   * **linux**
///     * by reading `/proc/sys/kernel/domainname`
///   * **windows**
///     * by calling win32 api `NetWkstaGetInfo`
///   ...
pub fn domainname() -> Result<String, Error> {
    _domainname()
}
