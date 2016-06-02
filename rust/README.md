# Minimal Rust project to access the [commercetools platform](http://dev.commercetools.com/) API.

This project contains:

- a mini-library `sphere` that could evolve into a real SDK in the future
- examples using the mini-library

## Sphere library

The `sphere` library contains the module [`sphere::auth`](/rust/src/auth.rs), used to retrieve an [access token](http://dev.commercetools.com/http-api-authorization.html) to use the API.

This library follows the Rust standard:

- unit tests are found next to the implementation.
- integration tests are found in ['/tests'](/rust/tests).
- code is formatted with [rustfmt](https://github.com/rust-lang-nursery/rustfmt).

## Prerequisites

[Rust](https://www.rust-lang.org/) and [Cargo](https://crates.io/install) must be installed. We advise you to use [rustup](https://github.com/rust-lang-nursery/rustup.rs) to install them.

## Hack
```
cargo test
```

## Example [fetch_products](/rust/examples/fetch_products.rs)
### Run the example

Usage:
```
cargo run --example fetch_products -- <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
```

To output some logs, [configure the `RUST_LOG` environment variable](http://doc.rust-lang.org/log/env_logger/index.html)

Example: to enable all logs for the `auth` module:
```
RUST_LOG=sphere::auth cargo run --example fetch_products -- <PROJECT_KEY> <CLIENT_ID> <CLIENT_SECRET>
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

The `sphere` library is not published to https://crates.io/ so you must link it locally.

Example of a `Cargo.toml`:
```
[package]
name = "my-rust-app"
version = "0.1.0"
authors = ["My name <my@email.com>"]

[dependencies]
hyper = "0.9"
rustc-serialize = "0.3"

[dependencies.sphere]
path = "<path to the sphere lib>"
```
