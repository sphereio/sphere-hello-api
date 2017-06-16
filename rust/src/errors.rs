
error_chain! {
    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // boxed as the error cause and wrapped in a new error.
    //
    // This section can be empty.
    foreign_links {
        Io(::std::io::Error);
        JsonSerialization(::serde_json::Error);
        Hyper(::hyper::Error);
        Uri(::hyper::error::UriError);
    }

    // Define additional `ErrorKind` variants.
    errors {
        UnexpectedStatus(msg: String, response: String) {
            description("unexpected http status")
            display("unexpected http status: {}. Response: '{:?}'", msg, response)
        }
    }
}
