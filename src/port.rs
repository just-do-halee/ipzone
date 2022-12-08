use super::*;
use std::{num::ParseIntError, str::FromStr};

#[inline]
pub fn from(source: impl Into<u16>) -> Port {
    Port(source.into())
}
#[inline]
pub fn from_str(s: &str) -> Result<u16, ParseIntError> {
    _from_str(s)
}
#[inline]
pub fn from_env(key: &str) -> Result<u16, FromEnvError> {
    _from_env(key)
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Port(pub u16);

impl From<Port> for u16 {
    #[inline]
    fn from(port: Port) -> Self {
        port.0
    }
}

impl From<u16> for Port {
    #[inline]
    fn from(port: u16) -> Self {
        Self(port)
    }
}
impl TryFrom<&str> for Port {
    type Error = ParseIntError;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Self(_from_str(s)?))
    }
}
impl FromStr for Port {
    type Err = ParseIntError;
    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
