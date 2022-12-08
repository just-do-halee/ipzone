use ipzone::prelude::*;
use std::env;

#[test]
fn it_works() {
    let localhost = ip::localhost().with([6060, 8080, 1234, 5678]);
    assert_eq!(
        localhost.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 6060),
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 8080),
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 1234),
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 5678),
        ]
    );
}

#[test]
fn it_works_for_str() {
    let ip = ip::from([186, 23, 123, 1]).with(["80".parse().unwrap(), 443, 123, 5678]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv4::new(186, 23, 123, 1).into(), 80),
            SocketAddr::new(Ipv4::new(186, 23, 123, 1).into(), 443),
            SocketAddr::new(Ipv4::new(186, 23, 123, 1).into(), 123),
            SocketAddr::new(Ipv4::new(186, 23, 123, 1).into(), 5678),
        ]
    );
}

#[test]
fn port_from_str() {
    let ip = ip::localhost().with([
        //
        80,
        port::from_str("4240").unwrap(),
        port::from_str("asdasd").unwrap_or(2020),
    ]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 80),
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 4240),
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 2020),
        ]
    );
}

#[test]
fn port_from_env() {
    env::set_var("PORT", "8080");
    let ip = ip::localhost().with([
        //
        80,
        port::from_env("PORT").unwrap(),
    ]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 80),
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 8080),
        ]
    );
}
