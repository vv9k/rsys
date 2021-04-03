#![cfg_attr(target_os = "macos", allow(dead_code))]
use super::Error;
use std::any::type_name;

pub fn trim_parse_map<T>(inp: &str) -> Result<T, Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    inp.trim().parse::<T>().map_err(|e| {
        Error::InvalidInputError(
            inp.to_string(),
            format!("cannot parse as '{}' - '{}'", type_name::<T>(), e),
        )
    })
}

pub fn next<'l, T, I>(iter: &mut I, src: &str) -> Result<T, Error>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
    I: Iterator<Item = &'l str>,
{
    if let Some(s) = iter.next() {
        return trim_parse_map(s);
    }

    Err(Error::InvalidInputError(
        src.to_string(),
        format!("there was no element of type {}", type_name::<T>()),
    ))
}
pub fn skip<I, T>(n: usize, iter: &mut I) -> &mut I
where
    I: Iterator<Item = T>,
{
    for _ in 0..n {
        iter.next();
    }
    iter
}
