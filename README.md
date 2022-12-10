# **`ipzone`**

Ipzone provides a simple and powerful IP architecture to Rust.

[![CI][ci-badge]][ci-url]
[![Crates.io][crates-badge]][crates-url]
[![Licensed][license-badge]][license-url]
[![Twitter][twitter-badge]][twitter-url]

[ci-badge]: https://github.com/just-do-halee/ipzone/actions/workflows/ci.yml/badge.svg
[crates-badge]: https://img.shields.io/crates/v/ipzone.svg?labelColor=383636
[license-badge]: https://img.shields.io/crates/l/ipzone?labelColor=383636
[twitter-badge]: https://img.shields.io/twitter/follow/do_halee?style=flat&logo=twitter&color=4a4646&labelColor=333131&label=just-do-halee
[ci-url]: https://github.com/just-do-halee/ipzone/actions
[twitter-url]: https://twitter.com/do_halee
[crates-url]: https://crates.io/crates/ipzone
[license-url]: https://github.com/just-do-halee/ipzone

| [Examples](./examples/) | [Docs](https://docs.rs/ipzone) | [Latest Note](./CHANGELOG.md) |

```toml
ipzone = "0.4.0"
```

## **`Examples`**

```rust
use ipzone::prelude::*;

let local: Address<3> = ip::localhost([6004, 7040, 8080]);
TcpListener::bind(local);

let address: Address<2> = ip::from([168, 159, 42, 9]).with([80, 443]);
TcpStream::connect(address);

let local = ip::localhost([port::from_env("PORT").unwrap_or(8080), 7020, 2020]);
TcpListener::bind(local);

let address = ip::from([186, 23, 123, 1, 0, 0, 0, 0]).with([80, 443]);
TcpStream::connect(address);

let local = ip::localhost([1234, 5678, port::from_str("9090").unwrap());
TcpListener::bind(local);

let address = ip::from_str("168.24.41.123").unwrap().with([80, 443]);
TcpStream::connect(address);

let local = ip::from_str("::1").unwrap().with([80, 443]);
TcpListener::bind(local);

let address = ip::from_env("IP").unwrap().with([80, 443]);
TcpStream::connect(address);

static LOCALHOST: Global<Address> = global(|| ip::localhost().with([8080]));
```

