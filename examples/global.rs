use std::net::{TcpListener, TcpStream};

use ipzone::prelude::*;

static LOCALHOST: Global<Address> = global(|| ip::localhost().with([8080]));

static NODE: Global<Address<2>> = global(|| {
    //
    ip::from([186, 23, 123, 1]).with([80, 443])
});

fn main() {
    TcpListener::bind(*LOCALHOST)
        .unwrap()
        .set_nonblocking(true)
        .unwrap();
    TcpStream::connect_timeout(&NODE.first(), std::time::Duration::from_secs(1))
        .unwrap()
        .set_nonblocking(true)
        .unwrap();
}
