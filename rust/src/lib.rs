#![recursion_limit = "1024"]

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;
extern crate chrono;

extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

pub use auth::Token;
pub use errors::*;
pub use region::*;

mod errors;
pub mod auth;
pub mod region;
pub mod client;
