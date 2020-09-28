#[allow(unused_imports)]
use rsys::{linux::cores, Result, Rsys};

#[cfg(target_os = "linux")]
fn main() -> Result<()> {
    let rsys = Rsys::new();
    println!("KERNEL VERSION - {}", rsys::linux::kernel_version()?);
    println!("HOSTNAME - {}", rsys::linux::hostname()?);
    println!("MEMORY - {:#?}", rsys.memory()?);
    println!("KERNEL_VERSION - {}", rsys.kernel_version()?);
    println!("{:#?}", rsys::linux::processor()?);
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() -> Result<()> {
    Ok(())
}
