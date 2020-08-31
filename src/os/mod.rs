pub mod linux;
pub mod macos;
use super::error::RsysError as Error;
use std::env;
use std::process::Command;
use std::str;

fn run(cmd: &mut Command) -> Result<String, Error> {
    match cmd.output() {
        Ok(out) => match str::from_utf8(&out.stdout) {
            Ok(out) => Ok(out.trim_end_matches('\n').to_string()),
            Err(e) => Err(Error::CommandParseError(e.to_string())),
        },
        Err(e) => Err(Error::CommandRunError(e.to_string())),
    }
}

pub fn hostname() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::hostname()
    } else if cfg!(target_os = "linux") {
        linux::hostname()
    } else {
        todo!()
    }
}

pub fn domainname() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::domainname()
    } else {
        todo!()
    }
}

pub fn ipv4(iface: &str) -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::ipv4(iface)
    } else if cfg!(target_os = "linux") {
        linux::ipv4(iface)
    } else {
        todo!()
    }
}

pub fn ipv6(iface: &str) -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::ipv6(iface)
    } else if cfg!(target_os = "linux") {
        linux::ipv6(iface)
    } else {
        todo!()
    }
}

pub fn mac(iface: &str) -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::mac(iface)
    } else {
        todo!()
    }
}

pub fn interfaces() -> Result<Vec<String>, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::interfaces()
    } else {
        todo!()
    }
}

pub fn cpu() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::cpu()
    } else if cfg!(target_os = "linux") {
        linux::cpu()
    } else {
        todo!()
    }
}

pub fn cpu_clock() -> Result<f32, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::cpu_clock()
    } else {
        todo!()
    }
}

pub fn arch() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::arch()
    } else if cfg!(target_os = "linux") {
        linux::arch()
    } else {
        todo!()
    }
}

pub fn uptime() -> Result<u64, Error> {
    if cfg!(target_os = "macos") {
        macos::uptime()
    } else if cfg!(target_os = "linux") {
        linux::uptime()
    } else {
        todo!()
    }
}

pub fn memory() -> Result<usize, Error> {
    if cfg!(target_os = "macos") {
        macos::memory()
    } else if cfg!(target_os = "linux") {
        linux::memory()
    } else {
        todo!()
    }
}

pub fn default_iface() -> Result<String, Error> {
    if cfg!(target_os = "macos") {
        macos::default_iface()
    } else if cfg!(target_os = "linux") {
        linux::default_iface()
    } else {
        todo!()
    }
}

pub fn swap() -> Result<usize, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::swap()
    } else {
        todo!()
    }
}

pub fn cpu_cores() -> Result<u16, Error> {
    if cfg!(target_os = "macos") {
        todo!()
    } else if cfg!(target_os = "linux") {
        linux::cpu_cores()
    } else {
        todo!()
    }
}

pub fn os() -> String {
    env::consts::OS.to_string()
}
