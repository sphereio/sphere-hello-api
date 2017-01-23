
error_chain! {
    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // boxed as the error cause and wrapped in a new error.
    //
    // This section can be empty.
    foreign_links {
        Io(::std::io::Error);
        JsonDecoding(::rustc_serialize::json::DecoderError);
        JsonEncoding(::rustc_serialize::json::EncoderError);
        Hyper(::hyper::Error);
    }

    // Define additional `ErrorKind` variants.
    errors {
        UnexpectedStatus(msg: String, response: String) {
            description("unexpected http status")
            display("unexpected http status: {}. Response: '{:?}'", msg, response)
        }
    }
}
