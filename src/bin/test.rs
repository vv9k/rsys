use rsys::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", rsys::hostname()?);
    let iface = rsys::default_iface()?;
    println!("{}", rsys::ipv4(&iface)?);
    println!("{}", rsys::mac(&iface)?);
    println!("{}", rsys::cpu()?);
    println!("{}", rsys::arch()?);
    println!("{}", rsys::memory()?);
    println!("{}", rsys::uptime()?);
    println!("{:#?}", rsys::linux::interfaces()?);
    Ok(())
}
