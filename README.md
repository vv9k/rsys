# rsys
Crate for aquiring information about host machine and operating system
in a os-agnostic fashion.  
 
The common api and error type is available at the root of this crate for convienience
while os-specific functions are hidden in submodules corresponding to each os.  

Main goals are clear and easy api with as much of the api being os-agnostic.
  
## Example usage:
- `Cargo.toml`

```toml
[dependencies]
rsys = "0.1"
```

- `main.rs`

```rust
use rsys::*;

fn main() -> Result<(), Error> {
    // Common api
    let hostname = rsys::hostname()?;
    let iface = rsys::default_iface()?;
    let ip4 =  rsys::ipv4(&iface)?;
    let mac =  rsys::mac(&iface)?;
    let cpu = rsys::cpu()?;
    let arch = rsys::arch()?;
    let memory = rsys::memory()?;
    let uptime = rsys::uptime()?;
    let ifaces = rsys::interfaces()?;
    let swap = rsys::swap()?;
    let cpu_cores = rsys::cpu_cores()?;
    let cpu_clock = rsys::cpu_clock()?;
    
    // Linux only
    let kernel_version = rsys::linux::kernel_version()?;
    // Mac only
    let model = rsys::macos::model()?;
    Ok(())
}
```

## License
[**MIT**](https://gitlab.com/vvvxxx/rsys/-/blob/master/LICENSE)