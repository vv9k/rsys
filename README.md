# rsys
Crate for aquiring information about host machine and operating system
in a os-agnostic fashion.  
 
The common api is available through Rsys struct which compiles conditionally with
required methods. The error type is available at the root of this crate for convienience
while os-specific functions are hidden in submodules corresponding to each os.  

Main goals are clear and easy api with as much of the api being os-agnostic.
  
## Example usage:
- `Cargo.toml`

```toml
[dependencies]
rsys = "0.3"
```

- `main.rs`
```rust
use rsys::{Error, Rsys};
fn main() -> Result<(), Error> {
    // You can either use api through Rsys object
    // for os-agnostic experience
    let rsys = Rsys::new();
    println!("HOSTNAME - {}", rsys.hostname()?);
    let iface = rsys.default_iface()?;
    println!("CPU - {}", rsys.cpu()?);
    println!("ARCH - {}", rsys.arch()?);
    println!("MEMORY - {} b", rsys.memory()?);
    println!("UPTIME - {} s", rsys.uptime()?);
    println!("SWAP - {}b", rsys.swap()?);
    println!("CPU CORES - {}", rsys.cpu_cores()?);
    println!("CPU CLOCK - {} MHz", rsys.cpu_clock()?);
    println!("IPv4 - {}", rsys.ipv4(&iface)?);
    println!("MAC - {}", rsys.mac(&iface)?);
    println!("INTERFACES - {:#?}", rsys.interfaces()?);
    
    // Or use functions in each module
    if cfg!(target_os = "linux") {
        println!("KERNEL VERSION - {}", rsys::linux::kernel_version()?);
        println!("HOSTNAME - {}", rsys::linux::hostname()?);
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
