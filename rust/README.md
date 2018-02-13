# Minimal Rust project to access the [commercetools platform](http://dev.commercetools.com/) API.

This project contains:

- a mini-library `commercetools` that could evolve into a real SDK in the future
- examples using the mini-library

## Sphere library

The `commercetools` library contains the module [`commercetools::auth`](/rust/src/auth.rs), used to retrieve an [access token](http://dev.commercetools.com/http-api-authorization.html) to use the API.

This library follows the Rust standard:

- unit tests are found next to the implementation.
- integration tests are found in ['/tests'](/rust/tests).
- code is formatted with [rustfmt](https://github.com/rust-lang-nursery/rustfmt).

## Prerequisites

[Rust](https://www.rust-lang.org/) and [Cargo](https://crates.io/install) must be installed. We advise you to use [rustup](https://github.com/rust-lang-nursery/rustup.rs) to install them.

## Hack
### Run the tests
```
cargo test
```

### Generate the documentation
```
cargo doc
```
The documentation is available in `target/doc/commercetools`.

### Format the code
Only once:
```
cargo install rustfmt
```

```
cargo fmt
```

## Example [fetch_products](/rust/examples/fetch_products.rs)
### Run the example

Help:
```
cargo run --example fetch_products -- --help
```

Usage:
```
cargo run --example fetch_products -- <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```

To output some logs, [configure the `RUST_LOG` environment variable](http://doc.rust-lang.org/log/env_logger/index.html)

Example: to enable all logs for the `auth` module:
```
RUST_LOG=commercetools::auth cargo run --example fetch_products -- <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```

### Build a release for this example

Create an executable from an example:
```
cargo build --example fetch_products
```

Run the executable
```
./target/debug/examples/fetch_products <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```

## Use the library from another Rust application

The `commercetools` library is not published to https://crates.io/ so you must link it locally.

Example of a `Cargo.toml`:
```
[package]
name = "my-rust-app"
version = "0.1.0"
authors = ["My name <my@email.com>"]

[dependencies]
hyper = "0.10"
serde = "0.9"
serde_derive = "0.9"
serde_json = "0.9"

[dependencies.commercetools]
path = "<path to the commercetools lib>"
```

## Technical choices
### Current version

The `commercetools` library uses:

- [hyper](http://hyper.rs/) as http client.
- [serde_json](https://docs.serde.rs/serde_json/) as JSON encoding / decoding parser.
- [log](https://doc.rust-lang.org/log) as logging facade.
- [failure](https://docs.rs/failure) to create errors with stack traces.

### Roadmap

- move from blocking IO to non-blocking IO. The next major version of [hyper](http://hyper.rs/hyper/master/hyper/client/index.html) will allow that.
