use failure::*;
use std::result;

pub type Result<T> = result::Result<T, Error>;

// use pattern https://boats.gitlab.io/failure/custom-fail.html

#[derive(Fail, Debug)]
#[fail(
    display = "unexpected http status: {}. Response: '{:?}'",
    msg, response
)]
pub struct UnexpectedStatus {
    msg: String,
    response: String,
}

impl UnexpectedStatus {
    pub fn new(msg: String, response: String) -> UnexpectedStatus {
        UnexpectedStatus {
            msg,
            response,
        }
    }
}
