use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

use ipzone::prelude::*;

fn main() {
    std::env::set_var("PORT", "8080");

    let localhost = ip::localhost().with([
        //
        1234,
        port::from_env("PORT").unwrap(),
        port::from_env("PORT2").unwrap_or(5050),
    ]);
    assert_eq!(
        localhost.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5050),
        ]
    );

    let ip = ip::from([186, 23, 123, 1]).with([
        //
        80,
        port::from_str("123").unwrap(),
        port::from_env("PORT").unwrap(),
    ]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv4Addr::new(186, 23, 123, 1).into(), 80),
            SocketAddr::new(Ipv4Addr::new(186, 23, 123, 1).into(), 123),
            SocketAddr::new(Ipv4Addr::new(186, 23, 123, 1).into(), 8080),
        ]
    );

    let ip2 = ip::from_str("::ffff:aabb:ccdd").unwrap().with([80, 443]);
    assert_eq!(
        ip2.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(
                Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xaabb, 0xccdd).into(),
                80
            ),
            SocketAddr::new(
                Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xaabb, 0xccdd).into(),
                443
            ),
        ]
    );

    std::env::set_var("IP", "168.32.12.1");
    let ip3 = ip::from_env("IP").unwrap().with([80, 443]);
    assert_eq!(
        ip3.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv4Addr::new(168, 32, 12, 1).into(), 80),
            SocketAddr::new(Ipv4Addr::new(168, 32, 12, 1).into(), 443),
        ]
    );
}
