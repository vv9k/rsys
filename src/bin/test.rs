#[allow(unused_imports)]
use rsys::{Result, Rsys};

fn main() -> Result<()> {
    // You can either use api through Rsys object
    // for os-agnostic experience
    let rsys = Rsys::new();
    println!("HOSTNAME - {}", rsys.hostname()?);
    println!("CPU - {}", rsys.cpu()?);
    println!("CPU CORES - {}", rsys.cpu_cores()?);
    println!("CPU CLOCK - {}MHz", rsys.cpu_clock()?);
    println!("UPTIME - {}s", rsys.uptime()?);
    println!("ARCH - {}", rsys.arch()?);
    println!("SWAP TOTAL - {}b", rsys.swap_total()?);
    println!("SWAP FREE - {}b", rsys.swap_free()?);
    println!("MEMORY TOTAL - {}b", rsys.memory_total()?);
    println!("MEMORY FREE - {}b", rsys.memory_free()?);

    #[cfg(target_os = "linux")]
    {
        println!("KERNEL VERSION - {}", rsys::linux::kernel_release()?);
        println!("HOSTNAME - {}", rsys::linux::hostname()?);
        println!("KERNEL_VERSION - {}", rsys.kernel_release()?);
        println!("{:#?}", rsys::linux::cpu::processor()?);
        println!("MOUNTS - {:#?}", rsys::linux::mounts::mounts()?);
    }
    Ok(())
}
