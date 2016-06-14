use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::{Headers, Authorization, Bearer};
use super::region::Region;

pub struct CtpClient {
    api_url: String,
    auth_url: String,
    project_key: String,
    client_id: String,
    client_secret: String,
}

impl CtpClient {
    pub fn new(region: &Region,
               project_key: &str,
               client_id: &str,
               client_secret: &str) -> CtpClient {
        CtpClient {
            api_url: region.api_url(),
            auth_url: region.auth_url(),
            project_key: project_key.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        }
    }

    pub fn request(&self, uri: &str) -> String {
        let token = super::auth::retrieve_token(&self.auth_url, &self.project_key, &self.client_id, &self.client_secret).unwrap();
        let access_token = token.access_token();

        let mut headers = Headers::new();
        headers.set(Authorization(Bearer { token: access_token }));

        let client = Client::new();

        let uri = format!("{}/{}{}", self.api_url, self.project_key, uri);
        let mut projets_res = client.get(&uri)
            .header(Connection::close())
            .headers(headers)
            .send()
            .unwrap();

        let mut body = String::new();
        projets_res.read_to_string(&mut body).unwrap();
        body
    }

}
