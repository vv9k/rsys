# rsys
[![Build Status](https://travis-ci.com/wojciechkepka/rsys.svg?branch=master)](https://travis-ci.com/wojciechkepka/rsys)
[![crates.io](https://img.shields.io/crates/v/rsys)](https://crates.io/crates/rsys)
[![crates.io](https://img.shields.io/crates/l/rsys)](https://github.com/wojciechkepka/rsys/blob/master/LICENSE)
[![Docs](https://img.shields.io/badge/docs-master-brightgreen)](https://docs.rs/rsys)  
Crate for aquiring information about host machine and operating system
in a os-agnostic fashion.  
 
The common api is available through Rsys struct which compiles conditionally with
required methods. The error and result type is available at the root of this crate for convienience
while all the methods exposed by Rsys struct are also available in each os module.  

Main goals are clear and easy api with as much of the api being os-agnostic.
  
## Example usage:
- `Cargo.toml`

```toml
[dependencies]
rsys = "0.4"
```

- `main.rs`
```rust
use rsys::{Rsys, Result};
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

    
    // Or use functions in each module
    if cfg!(target_os = "linux") {
        println!("KERNEL VERSION - {}", rsys::linux::kernel_version()?);
        println!("HOSTNAME - {}", rsys::linux::hostname()?);

        // Os-specific functions are also available as methods
        println!("MEMORY - {:#?}", rsys.memory()?);
        println!("KERNEL_VERSION - {:#?}", rsys.kernel_version()?);
    }
    Ok(())
}
```

## TODO
 - [ ] Finish macos common api
 - [ ] Finish windows common api
 - [ ] Add async feature for async file reads and commands etc...

## License
[**MIT**](https://gitlab.com/vvvxxx/rsys/-/blob/master/LICENSE)
