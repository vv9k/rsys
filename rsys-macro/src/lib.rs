extern crate proc_macro;
use proc_macro::TokenStream;

const PUB_CRATE_PREFIX: &str = "pub(crate) struct ";

#[proc_macro_derive(OsImpl)]
pub fn os_impl(item: TokenStream) -> TokenStream {
    let s = format!("{}", item);
    if let Some(prefix) = s.strip_prefix(PUB_CRATE_PREFIX) {
        let name = prefix.split_ascii_whitespace().next().unwrap();

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

            fn domain_name(&self) -> Result<String> {{
                domain_name()
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
        
        }}
        ",
            &name, &name
        )
        .parse()
        .unwrap();
    }
    TokenStream::new()
}
