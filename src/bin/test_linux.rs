#[allow(unused_imports)]
use rsys::{Result, Rsys};

#[cfg(target_os = "linux")]
fn main() -> Result<()> {
    let rsys = Rsys::new();
    println!("KERNEL VERSION - {}", rsys::linux::kernel_release()?);
    println!("HOSTNAME - {}", rsys::linux::hostname()?);
    println!("KERNEL_VERSION - {}", rsys.kernel_release()?);
    println!("{:#?}", rsys::linux::cpu::processor()?);
    println!("MOUNTS - {:#?}", rsys::linux::misc::mounts()?);
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() -> Result<()> {
    Ok(())
}
