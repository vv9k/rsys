#[allow(unused_imports)]
use rsys::*;

fn main() -> Result<(), Error> {
    println!("{}", rsys::hostname()?);
    let iface = rsys::default_iface()?;
    println!("{}", rsys::ipv4(&iface)?);
    println!("{}", rsys::mac(&iface)?);
    println!("{}", rsys::cpu()?);
    println!("{}", rsys::arch()?);
    println!("{}", rsys::memory()?);
    println!("{}", rsys::uptime()?);
    println!("{:#?}", rsys::interfaces()?);
    println!("{}", rsys::swap()?);
    println!("{}", rsys::cpu_cores()?);
    println!("{}", rsys::cpu_clock()?);
    println!("{}", rsys::linux::kernel_version()?);
    Ok(())
}
