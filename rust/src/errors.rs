use std::io::Error as IoError;
use rustc_serialize::json::DecoderError;
use hyper::client::Response;
use hyper::Error as HyperError;

error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    types {
        Error, ErrorKind, ChainErr, Result;
    }

    // Automatic conversions between this error chain and other
    // error chains.
    //
    // This section can be empty.
    links {
    }

    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // boxed as the error cause and wrapped in a new error.
    //
    // This section can be empty.
    foreign_links {
        IoError, Io, "IO error";
        DecoderError, Json, "json";
        HyperError, Hyper, "http";
    }

    // Define additional `ErrorKind` variants.
    errors {
        UnexpectedStatus(msg: String, response: Response) {
            description("unexpected http status")
            display("unexpected http status: {}. Reponse: '{:?}'", msg, response)
        }
    }
}
