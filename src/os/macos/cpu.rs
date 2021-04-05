use crate::macos::system::*;
use crate::{Error, Result};

use goblin::mach::cputype::{
    CPU_TYPE_ALPHA, CPU_TYPE_ARM, CPU_TYPE_ARM64, CPU_TYPE_I860, CPU_TYPE_MIPS, CPU_TYPE_POWERPC, CPU_TYPE_POWERPC64,
    CPU_TYPE_SPARC, CPU_TYPE_X86, CPU_TYPE_X86_64,
};
use sysctl::CtlValue;

pub fn arch() -> Result<String> {
    let cpu_type = match sysctl(SYSCTL_CPU_TYPE)? {
        CtlValue::Int(ty) => Ok(ty),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }? as u32;

    Ok(match cpu_type {
        CPU_TYPE_ALPHA => "alpha",
        CPU_TYPE_ARM => "arm32",
        CPU_TYPE_ARM64 => "arm64",
        CPU_TYPE_X86_64 => "x86_64",
        CPU_TYPE_X86 => "x86",
        CPU_TYPE_I860 => "i860",
        CPU_TYPE_MIPS => "mips",
        CPU_TYPE_POWERPC => "ppc",
        CPU_TYPE_POWERPC64 => "ppc_64",
        CPU_TYPE_SPARC => "sparc",
        _ => "unknown",
    }
    .to_string())
}

pub fn cpu() -> Result<String> {
    match sysctl(SYSCTL_CPU)? {
        CtlValue::String(cpu) => Ok(cpu),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}

pub fn cpu_clock() -> Result<f32> {
    match sysctl(SYSCTL_CPU_FREQUENCY)? {
        CtlValue::S64(clock) => Ok((clock / 1_000_000) as f32),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}

pub fn cpu_cores() -> Result<u16> {
    match sysctl(SYSCTL_CPU_CORES)? {
        CtlValue::Int(cores) => Ok(cores as u16),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}

pub fn logical_cores() -> Result<u16> {
    match sysctl(SYSCTL_LOGICAL_CORES)? {
        CtlValue::S64(num) => Ok(num as u16),
        val => Err(Error::UnexpectedSysctlValue(val)),
    }
}
