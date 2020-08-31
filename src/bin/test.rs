#[allow(unused_imports)]
use rsys::*;

fn main() -> Result<(), Error> {
    println!("HOSTNAME - {}", rsys::hostname()?);
    let iface = rsys::default_iface()?;
    println!("CPU - {}", rsys::cpu()?);
    println!("ARCH - {}", rsys::arch()?);
    println!("MEMORY - {} b", rsys::memory()?);
    println!("UPTIME - {} s", rsys::uptime()?);
    println!("SWAP - {}b", rsys::swap()?);
    println!("CPU CORES - {}", rsys::cpu_cores()?);
    println!("CPU CLOCK - {} MHz", rsys::cpu_clock()?);
    println!("IPv4 - {}", rsys::ipv4(&iface)?);
    if cfg!(target_os = "linux") {
        println!("MAC - {}", rsys::mac(&iface)?);
        println!("KERNEL VERSION - {}", rsys::linux::kernel_version()?);
        println!("INTERFACES - {:#?}", rsys::interfaces()?);
    } else if cfg!(target_os = "macos") {
        println!("{}", rsys::macos::model()?);
    }
    Ok(())
}
