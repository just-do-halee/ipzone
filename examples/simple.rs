use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use ipzone::prelude::*;

fn main() {
    std::env::set_var("P1", "8080");

    let ports: [u16; 6] = port::concatn!(
        [1010, port::from_env!("P1"), port::from_env!("P2", 9090)],
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
