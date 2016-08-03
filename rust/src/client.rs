use std::io::Read;
use std::cell::RefCell;

use hyper::Client;
use hyper::client::RequestBuilder;
use hyper::method::Method;
use hyper::header::{Headers, Authorization, Bearer};
use hyper::client::response::Response;
use hyper::status::StatusCode;

use rustc_serialize::json;
use rustc_serialize::Decodable;

/// a commercetools client
pub struct CtpClient<'a> {
    api_url: &'a str,
    auth_url: &'a str,
    project_key: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    permissions: Vec<&'a str>,
    client: Client,
    token: RefCell<Option<::Token>>,
}

#[derive(Debug)]
pub struct CtpResponse {
    pub http_reponse: Response,
}

impl CtpResponse {
    pub fn new(http_reponse: Response) -> CtpResponse {
        CtpResponse { http_reponse: http_reponse }
    }

    pub fn status(&self) -> StatusCode {
        self.http_reponse.status
    }

    pub fn body_as_string(&mut self) -> ::Result<String> {
        let mut body = String::new();
        try!(self.http_reponse.read_to_string(&mut body));
        Ok(body)
    }

    pub fn body_as<R: Decodable>(&mut self) -> ::Result<R> {
        let body = try!(self.body_as_string());
        Ok(try!(json::decode::<R>(&body)))
    }
}

#[derive(Debug, RustcDecodable)]
pub struct PagedQueryResult<R: Decodable> {
    pub offset: u64,
    pub count: u64,
    pub total: Option<u64>,
    pub results: Vec<R>,
}

#[derive(Debug, RustcEncodable)]
pub struct GraphQLQuery<'a> {
    pub query: &'a str,
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
    pub fn new(region: &'a ::Region,
               project_key: &'a str,
               client_id: &'a str,
               client_secret: &'a str)
               -> CtpClient<'a> {
        CtpClient {
            api_url: region.api_url(),
            auth_url: region.auth_url(),
            project_key: project_key,
            client_id: client_id,
            client_secret: client_secret,
            permissions: vec!["manage_project"],
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

    pub fn with_permissions(mut self, permissions: &Vec<&'a str>) -> CtpClient<'a> {
        self.permissions = permissions.clone();
        self
    }

    // TODO (YaSi): avoid cloning the String on each call
    pub fn get_token(&self) -> ::Result<String> {
        let mut cache = self.token.borrow_mut();
        if cache.is_some() {
            let token = cache.as_ref().unwrap();
            if token.is_valid() {
                return Ok(token.access_token.clone());
            }
        }

        let new_token = try!(super::auth::retrieve_token(&self.client,
                                                         self.auth_url,
                                                         self.project_key,
                                                         self.client_id,
                                                         self.client_secret,
                                                         &self.permissions));
        *cache = Some(new_token.clone());
        Ok(new_token.access_token)
    }

    pub fn list<R: Decodable>(&self, resource: &str) -> ::Result<PagedQueryResult<R>> {
        let url = format!("/{}?withTotal=false", resource);
        let mut response = try!(self.get(&url));
        let body = try!(response.body_as_string());
        Ok(try!(json::decode::<PagedQueryResult<R>>(&body)))
    }

    pub fn get(&self, uri: &str) -> ::Result<CtpResponse> {
        self.request(Method::Get, uri)
            .and_then(send)
    }

    pub fn post(&self, uri: &str, body: &str) -> ::Result<CtpResponse> {
        self.request(Method::Post, uri)
            .map(|r| r.body(body))
            .and_then(send)
    }

    pub fn delete(&self, uri: &str) -> ::Result<CtpResponse> {
        self.request(Method::Delete, uri)
            .and_then(send)
    }

    /// sends a [GraphQL](http://graphql.org/) query
    /// To test the query, use:
    ///
    /// - in Europe: https://impex.sphere.io/graphiql
    /// - in US: https://impex.commercetools.co/graphiql
    pub fn graphql(&self, query: &str) -> ::Result<CtpResponse> {
        let body = try!(json::encode(&GraphQLQuery { query: query }));

        self.request(Method::Post, "/graphql")
            .map(|r| r.body(&body))
            .and_then(send)
    }

    pub fn request(&self, method: Method, uri: &str) -> ::Result<RequestBuilder> {
        let client = &self.client;

        let access_token = try!(self.get_token());
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer { token: access_token }));

        let uri = format!("{}/{}{}", self.api_url, self.project_key, uri);
        Ok(client.request(method, &uri).headers(headers))
    }
}

fn send(r: RequestBuilder) -> ::Result<CtpResponse> {
    Ok(try!(r.send().map(CtpResponse::new)))
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
