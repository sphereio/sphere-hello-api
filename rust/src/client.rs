use std::io::Read;
use std::cell::RefCell;

use hyper::Client;
use hyper::client::RequestBuilder;
use hyper::method::Method;
use hyper::header::{Headers, Authorization, Bearer};
use super::region::Region;
use super::auth::Token;

/// a commercetools client
pub struct CtpClient<'a> {
    api_url: &'a str,
    auth_url: &'a str,
    project_key: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    client: Client,
    token: RefCell<Option<Token>>,
}

impl<'a> CtpClient<'a> {
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
    /// let region = Region::Europe;
    /// let client = CtpClient::new(&region, "my project key", "my client id", "my client secret");
    /// ```
    pub fn new(region: &'a Region, project_key: &'a str, client_id: &'a str, client_secret: &'a str) -> CtpClient<'a> {
        CtpClient {
            api_url: region.api_url(),
            auth_url: region.auth_url(),
            project_key: project_key,
            client_id: client_id,
            client_secret: client_secret,
            client: Client::new(),
            token: RefCell::new(None),
        }
    }

    pub fn with_auth_url(mut self, auth_url: &'a str) -> CtpClient<'a> {
        self.auth_url = auth_url;
        self
    }

    pub fn with_api_url(mut self, api_url: &'a str) -> CtpClient<'a> {
        self.api_url = api_url;
        self
    }

    // TODO (YaSi): avoid cloning the String on each call
    pub fn get_token(&self) -> Result<String, String> {
        let mut cache = self.token.borrow_mut();
        if cache.is_some() {
            let token = cache.as_ref().unwrap();
            if token.is_valid() {
                return Ok(token.access_token.clone());
            }
        }

        super::auth::retrieve_token(&self.client,
                                    self.auth_url,
                                    self.project_key,
                                    self.client_id,
                                    self.client_secret)
            .map(|new_token| {
                *cache = Some(new_token.clone());
                new_token.access_token
            })
    }

    pub fn get(&self, uri: &str) -> Result<String, String> {
        self.request(Method::Get, uri)
            .and_then(send)
    }

    pub fn post(&self, uri: &str, body: &str) -> Result<String, String> {
        self.request(Method::Post, uri)
            .map(|r| r.body(body))
            .and_then(send)
    }

    // TODO: this method "leaks" hyper RequestBuilder
    pub fn request(&self, method: Method, uri: &str) -> Result<RequestBuilder, String> {
        let client = &self.client;

        self.get_token()
            .map(|access_token| {
                let mut headers = Headers::new();
                headers.set(Authorization(Bearer { token: access_token }));

                let uri = format!("{}/{}{}", self.api_url, self.project_key, uri);
                client.request(method, &uri)
                    .headers(headers)
            })
    }
}

fn send(r: RequestBuilder) -> Result<String, String> {
    r.send()
        .map_err(|err| err.to_string())
        .and_then(|mut projets_res| {
            let mut body = String::new();
            projets_res.read_to_string(&mut body)
                .map_err(|err| err.to_string())
                .map(|_| body)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::region::Region;

    #[test]
    fn new_client() {
        CtpClient::new(&Region::Europe, "project_key", "client_id", "client_secret");
        CtpClient::new(&Region::NorthAmerica,
                       "project_key",
                       "client_id",
                       "client_secret");
    }

    #[test]
    fn new_client_with_customized_url() {
        CtpClient::new(&Region::Europe, "project_key", "client_id", "client_secret").with_api_url("my_api_url");

        CtpClient::new(&Region::Europe, "project_key", "client_id", "client_secret").with_auth_url("my_auth_url");

        CtpClient::new(&Region::Europe, "project_key", "client_id", "client_secret")
            .with_api_url("my_api_url")
            .with_auth_url("my_auth_url");
    }
}
