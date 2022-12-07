use std::str::FromStr;

#[inline]
pub fn _from_str<T: FromStr>(s: &str) -> Result<T, T::Err> {
    s.parse::<T>()
}

#[derive(Debug, thiserror::Error)]
pub enum FromEnvError {
    #[error("env not found")]
    NotFound,
    #[error("parse error")]
    Parse,
}

#[inline]
pub fn _from_env<T: FromStr>(key: &str) -> Result<T, FromEnvError> {
    std::env::var_os(key)
        .ok_or(FromEnvError::NotFound)?
        .to_str()
        .ok_or(FromEnvError::NotFound)?
        .parse()
        .map_err(|_| FromEnvError::Parse)
}
