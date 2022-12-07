use ipzone::prelude::*;

#[test]
fn it_works() {
    let localhost = ip::localhost().with([6060, 8080]);

    assert_eq!(
        localhost.to_socket_addrs().unwrap().collect::<Vec<_>>(),
        vec![
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 6060),
            SocketAddr::new(Ip::V4(Ipv4::new(127, 0, 0, 1)), 8080),
        ]
    );
}
