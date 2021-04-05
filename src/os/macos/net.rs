use crate::Result;

pub fn default_iface() -> Result<String> {
    Ok("".to_string())
}

pub fn ipv4(_iface: &str) -> Result<String> {
    Ok("".to_string())
}

pub fn ipv6(_iface: &str) -> Result<String> {
    Ok("".to_string())
}

pub fn mac(_iface: &str) -> Result<String> {
    Ok("".to_string())
}

pub fn interfaces() -> Result<Vec<String>> {
    Ok(vec![])
}
