#![recursion_limit = "1024"]

extern crate hyper;
extern crate rustc_serialize;
extern crate chrono;
#[macro_use]
extern crate log;

#[macro_use]
extern crate error_chain;

pub use errors::*;
pub use region::*;
pub use auth::Token;

mod errors;
pub mod auth;
pub mod region;
pub mod client;
