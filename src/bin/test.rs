#[allow(unused_imports)]
use rsys::{Error, Rsys};

fn display() -> Result<(), Error> {
    let rsys = Rsys::new();
    println!("HOSTNAME - {}", rsys.hostname()?);
    let iface = rsys.default_iface()?;
    println!("CPU - {}", rsys.cpu()?);
    println!("ARCH - {}", rsys.arch()?);
    println!("TOTAL MEMORY - {} b", rsys.memory_total()?);
    println!("FREE MEMORY - {} b", rsys.memory_free()?);
    println!("UPTIME - {} s", rsys.uptime()?);
    println!("TOTAL SWAP - {}b", rsys.swap_total()?);
    println!("FREE SWAP - {}b", rsys.swap_total()?);
    println!("CPU CORES - {}", rsys.cpu_cores()?);
    println!("LOGICAL CORES - {}", rsys.logical_cores()?);
    println!("CPU CLOCK - {} MHz", rsys.cpu_clock()?);
    println!("IPv4 - {}", rsys.ipv4(&iface)?);
    println!("MAC - {}", rsys.mac(&iface)?);
    println!("INTERFACES - {:#?}", rsys.interfaces()?);

    // On linux
    //println!("MOUNTS - {:?}", rsys::linux::mounts()?);
    //println!("IFACES - {:?}", rsys::linux::ifaces()?);
    //println!("PIDS - {:?}", rsys::linux::pids()?);
    //println!("PROCESSES - {:#?}", rsys::linux::processes()?);
    //println!("SDA - {:#?}", rsys::linux::stat_block_device("sda")?);
    //println!("DM0 - {:#?}", rsys::linux::stat_device_mapper("dm-0")?);
    //println!("DM1 - {:#?}", rsys::linux::stat_device_mapper("dm-1")?);
    //println!("DM2 - {:#?}", rsys::linux::stat_device_mapper("dm-2")?);

    Ok(())
}

fn main() -> Result<(), Error> {
    if let Err(e) = display() {
        println!("{}", e);
    }
    Ok(())
}
