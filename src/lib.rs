#![cfg(not(doctest))]
#![doc = include_str!("../README.md")]

pub mod prelude {
    pub use super::{port, Localhost};
    pub use std::net::ToSocketAddrs;
}

use prelude::*;

use std::{
    array::IntoIter,
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

#[macro_use]
pub mod port;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Localhost<const N: usize>(pub [u16; N]);

impl<const N: usize> ToSocketAddrs for Localhost<N> {
    type Iter = IntoIter<SocketAddr, N>;
    fn to_socket_addrs(&self) -> io::Result<Self::Iter> {
        const IP: IpAddr = IpAddr::V4(Ipv4Addr::LOCALHOST);
        Ok(self.0.map(|port| SocketAddr::new(IP, port)).into_iter())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        std::env::set_var("P1", "8080");

        assert_eq!(
            Localhost([1010, from_env!("P1"), from_env!("P2", 9090)])
                .to_socket_addrs()
                .unwrap()
                .collect::<Vec<_>>(),
            vec![
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1010),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9090),
            ]
        );
    }

    #[test]
    fn it_works_for_json() {
        let local = Localhost(from_json!("../assets/ports.test.json"; 3));
        assert_eq!(local, Localhost([1010, 8080, 1234]));

        assert_eq!(
            local.to_socket_addrs().unwrap().collect::<Vec<_>>(),
            vec![
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1010),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234),
            ]
        );
    }

    #[test]
    fn it_works_for_concatn() {
        std::env::set_var("P1", "8080");

        let ports: [u16; 6] = port::concatn!(
            [1010, port::from_env!("P1"), port::from_env!("P2", 9090),],
            port::from_json!("../assets/ports.test.json"; 3),
        );

        assert_eq!(
            Localhost(ports)
                .to_socket_addrs()
                .unwrap()
                .collect::<Vec<_>>(),
            vec![
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1010),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9090),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1010),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080),
                SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1234),
            ]
        );
    }
}
