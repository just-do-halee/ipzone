#[macro_export]
macro_rules! from_env {
    ($env_str:expr $(,)*) => {
        std::env::var_os($env_str)
            .expect("Missing environment variable")
            .to_str()
            .expect("Invalid environment variable")
            .parse()
            .expect("Invalid port")
    };
    ($env_str:expr, $default:expr $(,)*) => {
        (|| std::env::var_os($env_str)?.to_str()?.parse().ok())().unwrap_or($default)
    };
}
pub use from_env;

#[cfg(any(test, feature = "json"))]
#[macro_export]
macro_rules! from_json {
    ($json_str:expr; $N:expr $(,)*) => {{
        let f = || -> [u16; $N] {
            let mut ports =
                serde_json::from_str::<Vec<u16>>(include_str!($json_str)).expect("Invalid JSON");
            assert!(ports.len() <= $N);
            ports.resize($N, 0);
            ports.try_into().unwrap()
        };
        f()
    }};
}
#[cfg(any(test, feature = "json"))]
pub use from_json;

#[macro_export]
macro_rules! concatn {
    ($($ports:expr),* $(,)*) => {{
        [$($ports),*].concat().try_into().unwrap()
    }};
}
pub use concatn;
