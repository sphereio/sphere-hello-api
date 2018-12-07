#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

pub use crate::auth::Token;
pub use crate::errors::*;
pub use crate::region::*;

pub mod auth;
pub mod client;
mod errors;
pub mod region;
