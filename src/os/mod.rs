pub mod linux;
pub mod macos;
use super::error::RsysError as Error;
use std::env;
use std::process::Command;
use std::str;

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
///   ...
pub fn hostname() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::hostname()
    } else if cfg!(target_os = "linux") {
        linux::hostname()
    } else {
        todo!()
    }
}

/// Returns time since boot in seconds.
///   * **linux**
///     * by reading `/proc/uptime`
///   * **macos**
///     * by calling `sysctl("kern.boottime")`
///   ...
pub fn uptime() -> Result<u64, Error> {
    if cfg!(target_os = "macos") {
        macos::uptime()
    } else if cfg!(target_os = "linux") {
        linux::uptime()
    } else {
        todo!()
    }
}

/// Returns operating system. Reexported `env::consts::OS` for convenience.
pub fn os() -> String {
    env::consts::OS.to_string()
}

/// Returns cpu architecture.
///   * **linux** and **macos**
///     * by calling `uname -m`
///   ...
pub fn arch() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::arch()
    } else if cfg!(target_os = "linux") {
        linux::arch()
    } else {
        todo!()
    }
}
/// Returns a cpu model name.
///   * **linux**
///     * by reading `/proc/cpuinfo`
///   * **macos**
///     * by calling `sysctl("machdep.cpu.brand_string")`
///   ...
pub fn cpu() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::cpu()
    } else if cfg!(target_os = "linux") {
        linux::cpu()
    } else {
        todo!()
    }
}

/// Returns clock speed of cpu.
///   * **linux**
///     * by reading `/proc/cpuinfo`
///   * **macos**
///     * by calling `sysctl("...")`
///   ...
pub fn cpu_clock() -> Result<f32, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::cpu_clock()
    } else {
        todo!()
    }
}

/// Returns cpu cores.
///   * **linux**
///     * by reading `/proc/cpuinfo`
///   * **macos**
///     * by calling `sysctl("hw.physicalcpu")`
///   ...
pub fn cpu_cores() -> Result<u16, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::cpu_cores()
    } else {
        todo!()
    }
}

/// Returns total ram memory.
///   * **linux**
///     * by reading `/proc/meminfo`
///   * **macos**
///     * by calling `sysctl("hw.memsize")`
///   ...
pub fn memory() -> Result<usize, Error> {
    if cfg!(target_os = "macos") {
        macos::memory()
    } else if cfg!(target_os = "linux") {
        linux::memory()
    } else {
        todo!()
    }
}

/// Returns swap size.
///   * **linux**
///     * by reading `/proc/meminfo`
///   * **macos**
///     * by calling `sysctl("hw.swapusage")`
///   ...
pub fn swap() -> Result<usize, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::swap()
    } else {
        todo!()
    }
}

/// Returns a default internet interface.
///   * **linux**
///     * by calling `route` and determining default route
///   * **macos**
///     * by calling `route get default`
///   ...
pub fn default_iface() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::default_iface()
    } else if cfg!(target_os = "linux") {
        linux::default_iface()
    } else {
        todo!()
    }
}

/// Returns IP address version 4 of interface iface.
///   * **linux**
///     * by calling `ip address show <iface>`
///   * **macos**
///     * by calling `ipconfig getifaddr <iface>`
///   ...
pub fn ipv4(iface: &str) -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::ipv4(iface)
    } else if cfg!(target_os = "linux") {
        linux::ipv4(iface)
    } else {
        todo!()
    }
}

/// Returns IP address version 6 of interface iface.
pub fn ipv6(iface: &str) -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::ipv6(iface)
    } else if cfg!(target_os = "linux") {
        linux::ipv6(iface)
    } else {
        todo!()
    }
}

/// Returns a MAC address of interface iface.
///   * **linux**
///     * by calling `ip address show <iface>`
///   ...
pub fn mac(iface: &str) -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::mac(iface)
    } else {
        todo!()
    }
}

/// Returns a vector of names of network interfaces that are available.
///   * **linux**
///     * by calling `ip address show`
///   ...
pub fn interfaces() -> Result<Vec<String>, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::interfaces()
    } else {
        todo!()
    }
}

/// Returns a domain name.
///   * **linux**
///     * by reading `/proc/sys/kernel/domainname`
///   ...
pub fn domainname() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::domainname()
    } else {
        todo!()
    }
}
