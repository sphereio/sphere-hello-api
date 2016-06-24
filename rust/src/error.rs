use std::io::Error as IoError;
use rustc_serialize::json::DecoderError;
use hyper::client::Response;
use hyper::Error as HyperError;
use self::Error::{Io, Json, Hyper};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnexpectedStatus(Response),
    Io(IoError),
    Json(DecoderError),
    Hyper(HyperError),
}

impl From<DecoderError> for Error {
    fn from(err: DecoderError) -> Error {
        Json(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Io(err)
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Hyper(err)
    }
}
