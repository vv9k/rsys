#[allow(unused_imports)]
use rsys::*;

fn display() -> Result<(), Error> {
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
    println!("MAC - {}", rsys::mac(&iface)?);
    println!("INTERFACES - {:#?}", rsys::interfaces()?);
    if cfg!(target_os = "linux") {
        println!("KERNEL VERSION - {}", rsys::linux::kernel_version()?);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    if let Err(e) = display() {
        println!("{}", e);
    }
    Ok(())
}
