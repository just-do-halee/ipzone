use ipzone::prelude::*;
use std::env;

#[test]
fn ipv4_array() {
    let ip = ip::from([186, 23, 123, 1]).with([80, 443]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv4::new(186, 23, 123, 1).into(), 80),
            SocketAddr::new(Ipv4::new(186, 23, 123, 1).into(), 443),
        ]
    );
}

#[test]
fn ipv6_array() {
    let ip = ip::from([186, 23, 123, 1, 0, 0, 0, 0]).with([80, 443]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv6::new(186, 23, 123, 1, 0, 0, 0, 0).into(), 80),
            SocketAddr::new(Ipv6::new(186, 23, 123, 1, 0, 0, 0, 0).into(), 443),
        ]
    );
}

#[test]
fn ipv4_u32() {
    let ip = ip::from(0x0a000001u32.to_be_bytes()).with([80, 443]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv4::new(10, 0, 0, 1).into(), 80),
            SocketAddr::new(Ipv4::new(10, 0, 0, 1).into(), 443),
        ]
    );
}

#[test]
fn ipv6_u128() {
    let ip = ip::from(0x00000000000000000000ffffaabbccddu128.to_be_bytes()).with([80, 443]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv6::new(0, 0, 0, 0, 0, 0xffff, 0xaabb, 0xccdd).into(), 80),
            SocketAddr::new(Ipv6::new(0, 0, 0, 0, 0, 0xffff, 0xaabb, 0xccdd).into(), 443),
        ]
    );
}

#[test]
fn ipv4_str() {
    let ip = ip::from_str("164.23.1.2").unwrap().with([80, 443]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv4::new(164, 23, 1, 2).into(), 80),
            SocketAddr::new(Ipv4::new(164, 23, 1, 2).into(), 443),
        ]
    );
}

#[test]
fn ipv6_str() {
    let ip = ip::from_str("::ffff:aabb:ccdd").unwrap().with([80, 443]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv6::new(0, 0, 0, 0, 0, 0xffff, 0xaabb, 0xccdd).into(), 80),
            SocketAddr::new(Ipv6::new(0, 0, 0, 0, 0, 0xffff, 0xaabb, 0xccdd).into(), 443),
        ]
    );
}

#[test]
fn ipv4_env() {
    env::set_var("IP", "127.0.0.1");
    let ip = ip::from_env("IP").unwrap().with([80, 443]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv4::new(127, 0, 0, 1).into(), 80),
            SocketAddr::new(Ipv4::new(127, 0, 0, 1).into(), 443),
        ]
    );
}

#[test]
fn ipv6_env() {
    env::set_var("IP", "::ffff:aabb:ccdd");
    let ip = ip::from_env("IP").unwrap().with([80, 443]);
    assert_eq!(
        ip.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ipv6::new(0, 0, 0, 0, 0, 0xffff, 0xaabb, 0xccdd).into(), 80),
            SocketAddr::new(Ipv6::new(0, 0, 0, 0, 0, 0xffff, 0xaabb, 0xccdd).into(), 443),
        ]
    );
}
