extern crate commercetools;
extern crate hyper;

use hyper::server::{Handler, Request, Response, Server};

fn with_server<H: Handler + 'static, R>(handle: H, test: &Fn(String) -> R) -> R {
    let mut server = Server::http("localhost:0").unwrap().handle(handle).unwrap();
    let url = format!("http://localhost:{}", server.socket.port());
    let result = test(url);
    server.close().unwrap();
    result
}

#[test]
fn auth_can_extract_oauth_token() {
    fn handle(_: Request, res: Response) {
        res.send(b"{\"access_token\": \"test\", \"expires_in\": 234}")
            .unwrap();
    }

    with_server(handle, &|url| {
        let client = reqwest::Client::new();
        let token = commercetools::auth::retrieve_token(
            &client,
            &url,
            "project_key",
            "client_id",
            "client_secret",
            &["permission"],
        );
        assert!(token.is_ok(), "token = {:?}", token);
    });
}
