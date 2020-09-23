extern crate proc_macro;
use proc_macro::TokenStream;

const PUB_CRATE_PREFIX: &str = "pub(crate) struct ";

#[proc_macro_derive(OsImpl)]
pub fn os_impl(item: TokenStream) -> TokenStream {
    let s = format!("{}", item);
    if s.starts_with(PUB_CRATE_PREFIX) {
        let name = &s[PUB_CRATE_PREFIX.len()..].split_ascii_whitespace().next().unwrap();

        return format!(
            "
        impl {} {{
            pub fn new() -> Self {{
                Self::default()
            }}
        }}
        impl OsImpl for {} {{
            fn hostname(&self) -> Result<String> {{
                hostname()
            }}
        
            fn uptime(&self) -> Result<u64> {{
                uptime()
            }}
        
            fn arch(&self) -> Result<String> {{
                arch()
            }}
        
            fn cpu(&self) -> Result<String> {{
                cpu()
            }}
        
            fn cpu_clock(&self) -> Result<f32> {{
                cpu_clock()
            }}
        
            fn cpu_cores(&self) -> Result<u16> {{
                cpu_cores()
            }}
        
            fn logical_cores(&self) -> Result<u16> {{
                logical_cores()
            }}

            fn memory_total(&self) -> Result<usize> {{
                memory_total()
            }}
        
            fn memory_free(&self) -> Result<usize> {{
                memory_free()
            }}
        
            fn swap_total(&self) -> Result<usize> {{
                swap_total()
            }}
        
            fn swap_free(&self) -> Result<usize> {{
                swap_free()
            }}
        
            fn default_iface(&self) -> Result<String> {{
                default_iface()
            }}
        
            fn ipv4(&self, iface: &str) -> Result<String> {{
                ipv4(iface)
            }}
        
            fn ipv6(&self, iface: &str) -> Result<String> {{
                ipv6(iface)
            }}
        
            fn mac(&self, iface: &str) -> Result<String> {{
                mac(iface)
            }}
        
            fn interfaces(&self) -> Result<Vec<String>> {{
                interfaces()
            }}
        
            fn domainname(&self) -> Result<String> {{
                domainname()
            }}
        }}
        ",
            &name, &name
        )
        .parse()
        .unwrap();
    }
    TokenStream::new()
}
