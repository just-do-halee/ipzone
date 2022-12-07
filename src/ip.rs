use super::*;
use std::net::AddrParseError;

#[inline]
pub fn from(source: impl Into<Ip>) -> Ip {
    source.into()
}
#[inline]
pub fn from_str(s: &str) -> Result<Ip, AddrParseError> {
    _from_str(s)
}
#[inline]
pub fn from_env(key: &str) -> Result<Ip, FromEnvError> {
    _from_env(key)
}

#[inline]
pub fn localhost() -> Ip {
    from([127, 0, 0, 1])
}
