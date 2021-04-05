use crate::Result;

use sysctl::Sysctl;

pub(crate) const SYSCTL_CPU: &str = "machdep.cpu.brand_string";
pub(crate) const SYSCTL_HOSTNAME: &str = "kern.hostname";
pub(crate) const SYSCTL_BOOTTIME: &str = "kern.boottime";
pub(crate) const SYSCTL_MODEL: &str = "hw.model";
pub(crate) const SYSCTL_MEMSIZE: &str = "hw.memsize";
pub(crate) const SYSCTL_USERMEM: &str = "hw.usermem";
pub(crate) const SYSCTL_CPU_FREQUENCY: &str = "hw.cpufrequency";
pub(crate) const SYSCTL_CPU_CORES: &str = "hw.physicalcpu";
pub(crate) const SYSCTL_CPU_TYPE: &str = "hw.cputype";
pub(crate) const SYSCTL_LOGICAL_CORES: &str = "hw.logicalcpu";
pub(crate) const SYSCTL_VM_SWAPUSAGE: &str = "vm.swapusage";

pub(crate) fn sysctl(property: &str) -> Result<sysctl::CtlValue> {
    Ok(sysctl::Ctl::new(property).unwrap().value().unwrap())
}
