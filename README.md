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
ipzone = "0.1.0"
```

## **`Examples`**

```rust
use ipzone::prelude::*;

const LOCAL: Localhost<3> = Localhost([6004, 7040, 8080]);

// Localhost has implemented ToSocketAddrs trait so
// can server.bind(LOCAL);
```

```rust
// From environment variables
let local = Localhost([6004, 7040, port::from_env!("PORT")]);
// unwrap_or(8080) version is port::from_env!("PORT", 8080)
```

```rust
// From json files (features = "json")
// -- ../ports.json --
// [ 1234, 5678, 9101, 4321 ]
let local = Localhost(port::from_json!("../ports.json"; 4));
// can truncate it, port::from_json("../ports.json"; 2) = [ 1234, 5678 ]
```

```rust
// Concatenating
let ports: [u16; 4] = port::concatn!(
        [
            [1234, port::from_env!("PORT")]
            port::from_json!("../ports.json"; 2)
        ]
    );

let local = Localhost(ports);
```

