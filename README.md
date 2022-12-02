# @pzzld/pzzld-gateway

[![Docker](https://github.com/FL03/pzzld-gateway/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/pzzld-gateway/actions/workflows/docker.yml)
[![Clippy](https://github.com/FL03/pzzld-gateway/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/pzzld-gateway/actions/workflows/clippy.yml)
[![Rust](https://github.com/FL03/pzzld-gateway/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/pzzld-gateway/actions/workflows/rust.yml)

## About


## Installation

Use Rust's built-in package manager [crates](https://crates.io/crates/package) to install *package*.

```bash
cargo install package
```

## Usage

- [crates.io](https://crates.io/crates/scsys)
- [docs.rs](https://docs.rs/scsys)

```rust
use pzzld_gateway::gateways::{convert_credentials, simple_region, Gateway};
use scsys::prelude::*;

fn main() {
  println!("{:?}", Message::<String>::default());
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
[MIT](https://choosealicense.com/licenses/mit/)