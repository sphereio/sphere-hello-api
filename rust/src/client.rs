use std::io::Read;

use hyper::Client;
use hyper::header::Connection;
use hyper::header::{Headers, Authorization, Bearer};
use super::region::Region;

/// a commercetools client
pub struct CtpClient {
    api_url: String,
    auth_url: String,
    project_key: String,
    client_id: String,
    client_secret: String,
}

impl CtpClient {
    /// Returns a commercetools client for the given arguments
    ///
    /// # Arguments
    ///
    /// * `region` - the world region the client should use
    /// * `project_key` - project key
    /// * `client_id` - client id
    /// * `client_secret` - client secret
    ///
    /// # Examples
    ///
    /// ```
    /// use commercetools::region::Region;
    /// use commercetools::client::CtpClient;
    ///
    /// let client = CtpClient::new(&Region::Europe, "my project key", "my client id", "my client secret");
    /// ```
    pub fn new(region: &Region, project_key: &str, client_id: &str, client_secret: &str) -> CtpClient {
        CtpClient {
            api_url: region.api_url().to_string(),
            auth_url: region.auth_url().to_string(),
            project_key: project_key.to_string(),
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
        }
    }

    pub fn request(&self, uri: &str) -> String {
        let token = super::auth::retrieve_token(&self.auth_url,
                                                &self.project_key,
                                                &self.client_id,
                                                &self.client_secret)
            .unwrap();
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
