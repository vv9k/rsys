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
            fn hostname(&self) -> Result<String, Error> {{
                hostname()
            }}
        
            fn uptime(&self) -> Result<u64, Error> {{
                uptime()
            }}
        
            fn arch(&self) -> Result<String, Error> {{
                arch()
            }}
        
            fn cpu(&self) -> Result<String, Error> {{
                cpu()
            }}
        
            fn cpu_clock(&self) -> Result<f32, Error> {{
                cpu_clock()
            }}
        
            fn cpu_cores(&self) -> Result<u16, Error> {{
                cpu_cores()
            }}
        
            fn logical_cores(&self) -> Result<u16, Error> {{
                logical_cores()
            }}
        
            fn memory(&self) -> Result<usize, Error> {{
                memory()
            }}
        
            fn swap(&self) -> Result<usize, Error> {{
                swap()
            }}
        
            fn default_iface(&self) -> Result<String, Error> {{
                default_iface()
            }}
        
            fn ipv4(&self, iface: &str) -> Result<String, Error> {{
                ipv4(iface)
            }}
        
            fn ipv6(&self, iface: &str) -> Result<String, Error> {{
                ipv6(iface)
            }}
        
            fn mac(&self, iface: &str) -> Result<String, Error> {{
                mac(iface)
            }}
        
            fn interfaces(&self) -> Result<Vec<String>, Error> {{
                interfaces()
            }}
        
            fn domainname(&self) -> Result<String, Error> {{
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
