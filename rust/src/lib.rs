#![recursion_limit = "1024"]

extern crate chrono;
extern crate hyper;
extern crate hyper_native_tls;

extern crate serde;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate log;

extern crate failure;
extern crate failure_derive;

pub use auth::Token;
pub use errors::*;
pub use region::*;

mod errors;
pub mod auth;
pub mod region;
pub mod client;
