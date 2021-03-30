#[allow(unused_imports)]
use rsys::{Result, Rsys};

#[cfg(target_os = "linux")]
fn main() -> Result<()> {
    let rsys = Rsys::new();
    println!("KERNEL VERSION - {}", rsys::linux::kernel_release()?);
    println!("HOSTNAME - {}", rsys::linux::hostname()?);
    println!("MEMORY - {:#?}", rsys.memory()?);
    println!("KERNEL_VERSION - {}", rsys.kernel_release()?);
    println!("{:#?}", rsys::linux::cpu::processor()?);
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() -> Result<()> {
    Ok(())
}
