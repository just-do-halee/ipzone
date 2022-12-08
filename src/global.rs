use once_cell::sync::Lazy;

pub type Global<T> = Lazy<T>;

#[inline]
pub const fn global<T>(f: fn() -> T) -> Global<T> {
    Global::new(f)
}
