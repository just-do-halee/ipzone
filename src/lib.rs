#![cfg(not(doctest))]
#![doc = include_str!("../README.md")]

#[macro_use]
mod global;
mod address;

pub mod ip;
pub mod port;

pub mod prelude {
    pub use super::{ip, port};

    pub use super::{address::*, cmn::FromEnvError, global::*};

    pub use std::net::IpAddr as Ip;
    pub use std::net::Ipv4Addr as Ipv4;
    pub use std::net::Ipv6Addr as Ipv6;
    pub use std::net::SocketAddr;
    pub use std::net::ToSocketAddrs;
}

// -----------
//
mod cmn;
use cmn::*;

use port::*;
use prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    use std::net::{Ipv6Addr, SocketAddr};

    #[test]
    fn it_works() {
        std::env::set_var("IP", "::ffff:192.1.56.10");
        std::env::set_var("PORT", "8080");

        let address = ip::from_env("IP").unwrap().with([
            80,
            port::from_str("123").unwrap(),
            port::from_env("PORT").unwrap(),
        ]);

        assert_eq!(
            address.to_socket_addrs().unwrap().collect::<Vec<_>>(),
            vec![
                SocketAddr::new(
                    Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc001, 0x380a).into(),
                    80
                ),
                SocketAddr::new(
                    Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc001, 0x380a).into(),
                    123
                ),
                SocketAddr::new(
                    Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc001, 0x380a).into(),
                    8080
                ),
            ]
        );
    }
}
