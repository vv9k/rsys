use super::{run, Command, VM_PAGESIZE};
use crate::Result;

pub(crate) fn sysctl(property: &str) -> Result<String> {
    run(Command::new("sysctl").arg("-n").arg(property))
}

pub(crate) fn vm_pagesize() -> Result<u32> {
    Ok(sysctl(VM_PAGESIZE)?
        .parse::<u32>()
        .map_err(|e| Error::CommandParseError(e.to_string()))?)
}
