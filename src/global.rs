use once_cell::sync::Lazy;

pub type Global<T> = Lazy<T>;

#[macro_export]
macro_rules! global {
    ($e:expr) => {
        Global::new(|| $e);
    };
}

#[inline]
pub const fn global<T>(f: fn() -> T) -> Global<T> {
    Global::new(f)
}
