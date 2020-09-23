#[allow(unused_imports)]
use rsys::{Result, Rsys};

fn main() -> Result<()> {
    // You can either use api through Rsys object
    // for os-agnostic experience
    let rsys = Rsys::new();
    println!("HOSTNAME - {}", rsys.hostname()?);
    let iface = rsys.default_iface()?;
    println!("CPU - {}", rsys.cpu()?);
    println!("ARCH - {}", rsys.arch()?);
    println!("MEMORY TOTAL - {}b", rsys.memory_total()?);
    println!("UPTIME - {}s", rsys.uptime()?);
    println!("SWAP TOTAL - {}b", rsys.swap_total()?);
    println!("CPU CORES - {}", rsys.cpu_cores()?);
    println!("CPU CLOCK - {}MHz", rsys.cpu_clock()?);
    println!("IPv4 - {}", rsys.ipv4(&iface)?);
    println!("MAC - {}", rsys.mac(&iface)?);
    println!("INTERFACES - {:#?}", rsys.interfaces()?);
    Ok(())
}
