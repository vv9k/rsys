use super::{run, Error};
use std::process::Command;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

const SYSCTL_CPU: &str = "machdep.cpu.brand_string";
const SYSCTL_HOSTNAME: &str = "kern.hostname";
const SYSCTL_BOOTTIME: &str = "kern.boottime";
const SYSCTL_MODEL: &str = "hw.model";
const SYSCTL_MEMSIZE: &str = "hw.memsize";
const CPU_FREQUENCY: &str = "hw.cpufrequency";
const CPU_CORES: &str = "hw.physicalcpu";
const VM_PAGESIZE: &str = "vm.pagesize";

const INTERFACE: &str = "interface: ";
const INTERFACE_LEN: usize = INTERFACE.len();
const SYSCTL_BOOTTIME_LEN: usize = "{ sec = ".len();
const PAGES_ACTIVE: &str = "Pages active:";
const PAGES_INACTIVE: &str = "Pages inactive:";

fn sysctl(property: &str) -> Result<String, Error> {
    run(Command::new("sysctl").arg("-n").arg(property))
}

fn vm_pagesize() -> Result<u32, Error> {
    Ok(sysctl(VM_PAGESIZE)?.parse::<u32>().map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn default_iface() -> Result<String, Error> {
    let out = run(Command::new("route").arg("get").arg("default"))?;
    if let Some(ifc_line) = out.split('\n').filter(|l| l.trim().starts_with(INTERFACE)).next() {
        return Ok(ifc_line.trim()[INTERFACE_LEN..].trim_end_matches('\n').to_string());
    }

    Ok("".to_string())
}

pub(crate) fn hostname() -> Result<String, Error> {
    sysctl(SYSCTL_HOSTNAME)
}

pub(crate) fn ipv4(iface: &str) -> Result<String, Error> {
    run(Command::new("ipconfig").arg("getifaddr").arg(iface))
}

pub(crate) fn ipv6(_iface: &str) -> Result<String, Error> {
    todo!()
}

pub(crate) fn cpu() -> Result<String, Error> {
    sysctl(SYSCTL_CPU)
}

pub(crate) fn cpu_cores() -> Result<u16, Error> {
    Ok(sysctl(CPU_CORES)?.parse::<u16>().map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn cpu_clock() -> Result<f32, Error> {
    Ok(sysctl(CPU_FREQUENCY)?.parse::<u64>().map_err(|e| Error::CommandParseError(e.to_string())).map(|v| v as f32)?)
}

pub(crate) fn arch() -> Result<String, Error> {
    run(Command::new("uname").arg("-m"))
}

pub(crate) fn memory() -> Result<usize, Error> {
    Ok(sysctl(SYSCTL_MEMSIZE)?.parse::<usize>().map_err(|e| Error::CommandParseError(e.to_string()))?)
}

pub(crate) fn uptime() -> Result<u64, Error> {
    let boot = sysctl(SYSCTL_BOOTTIME)?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| Error::TimeError(e.to_string()))?.as_secs();
    let boottime = boot[SYSCTL_BOOTTIME_LEN..SYSCTL_BOOTTIME_LEN + format!("{}", now).len()]
        .parse::<u64>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?;
    Ok(now - boottime)
}

pub(crate) fn swap() -> Result<usize, Error> {
    let (mut active, mut inactive) = (0, 0);
    let (mut was_active, mut was_inactive) = (false, false);
    let mut pagesize = vm_pagesize()?;
    let mut cmd = Command::new("vm_stat");
    for line in run(&mut cmd)?.split('\n') {
        if line.starts_with(PAGES_ACTIVE) {
            active = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<u64>()
                .map_err(|e| Error::CommandParseError(e.to_string()))?;
            was_active = true;
        }
        if line.starts_with(PAGES_INACTIVE) {
            inactive = line
                .split_ascii_whitespace()
                .last()
                .unwrap()
                .parse::<u64>()
                .map_err(|e| Error::CommandParseError(e.to_string()))?;
            was_inactive = true;
        }
        if was_active && was_inactive {
            break;
        }
    }

    Ok(((active + inactive) * pagesize as u64) as usize)
}

/// Returns a model of host machine.
pub fn model() -> Result<String, Error> {
    sysctl(SYSCTL_MODEL)
}
