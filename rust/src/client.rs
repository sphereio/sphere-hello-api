use hyper::client::response::Response;
use hyper::client::RequestBuilder;
use hyper::header::Headers;
use hyper::method::Method;
use hyper::net::HttpsConnector;
use hyper::status::StatusCode;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use serde::de::DeserializeOwned;

use serde_json;
use std::io::Read;

/// a commercetools client
pub struct CtpClient<'a> {
    api_url: &'a str,
    auth_url: &'a str,
    project_key: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
    permissions: Vec<&'a str>,
    client: Client,
    token: Option<crate::Token>,
}

#[derive(Debug)]
pub struct CtpResponse {
    pub http_response: Response,
}

impl CtpResponse {
    pub fn new(http_response: Response) -> CtpResponse {
        CtpResponse {
            http_response,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.http_response.status
    }

    pub fn body_as_string(&mut self) -> crate::Result<String> {
        let mut body = String::new();
        self.http_response.read_to_string(&mut body)?;
        Ok(body)
    }

    pub fn body_as<R: DeserializeOwned>(&mut self) -> crate::Result<R> {
        let body = self.body_as_string()?;
        Ok(serde_json::from_str::<R>(&body)?)
    }
}

#[derive(Debug, Deserialize)]
pub struct PagedQueryResult<R> {
    pub offset: u64,
    pub count: u64,
    pub total: Option<u64>,
    pub results: Vec<R>,
}

#[derive(Debug, Serialize)]
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
    pub fn new<REG>(
        region: &REG,
        project_key: &'a str,
        client_id: &'a str,
        client_secret: &'a str,
    ) -> CtpClient<'a>
    where
        REG: crate::HasApiUrl<'a> + crate::HasAuthUrl<'a>,
    {
        let client =
            if region.api_url().starts_with("https") || region.auth_url().starts_with("https") {
                let ssl = NativeTlsClient::new().unwrap();
                let connector = HttpsConnector::new(ssl);
                Client::with_connector(connector)
            } else {
                Client::new()
            };

        CtpClient {
            api_url: region.api_url(),
            auth_url: region.auth_url(),
            project_key,
            client_id,
            client_secret,
            permissions: vec!["manage_project"],
            client,
            token: None,
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

    pub fn with_permissions(mut self, permissions: &[&'a str]) -> CtpClient<'a> {
        self.permissions = permissions.to_vec();
        self
    }

    // TODO (YaSi): avoid cloning the String on each call
    pub fn get_token(&mut self) -> crate::Result<Vec<u8>> {
        if let Some(ref token) = self.token {
            if token.is_valid() {
                return Ok(token.bearer_token.clone());
            }
        }

        let new_token = super::auth::retrieve_token(
            &self.client,
            self.auth_url,
            self.project_key,
            self.client_id,
            self.client_secret,
            &self.permissions,
        )?;
        self.token = Some(new_token.clone());
        Ok(new_token.bearer_token)
    }

    pub fn list<R: DeserializeOwned>(
        &mut self,
        resource: &str,
    ) -> crate::Result<PagedQueryResult<R>> {
        let url = format!("/{}?withTotal=false", resource);
        let body = self.get(&url)?.body_as_string()?;
        Ok(serde_json::from_str::<PagedQueryResult<R>>(&body)?)
    }

    pub fn get(&mut self, uri: &str) -> crate::Result<CtpResponse> {
        self.request(Method::Get, uri).and_then(send)
    }

    pub fn post(&mut self, uri: &str, body: &str) -> crate::Result<CtpResponse> {
        self.request(Method::Post, uri)
            .map(|r| r.body(body))
            .and_then(send)
    }

    pub fn delete(&mut self, uri: &str) -> crate::Result<CtpResponse> {
        self.request(Method::Delete, uri).and_then(send)
    }

    /// sends a [GraphQL](http://graphql.org/) query
    /// To test the query, use:
    ///
    /// - in Europe: https://impex.sphere.io/graphiql
    /// - in US: https://impex.commercetools.co/graphiql
    pub fn graphql(&mut self, query: &str) -> crate::Result<CtpResponse> {
        let body = serde_json::to_string(&GraphQLQuery { query })?;

        self.request(Method::Post, "/graphql")
            .map(|r| r.body(&body))
            .and_then(send)
    }

    pub fn request(&mut self, method: Method, uri: &str) -> crate::Result<RequestBuilder> {
        let bearer_token = self.get_token()?;
        let mut headers = Headers::new();
        headers.set_raw("Authorization", vec![bearer_token]);
        let uri = format!("{}/{}{}", self.api_url, self.project_key, uri);
        let client = &self.client;
        Ok(client.request(method, &uri).headers(headers))
    }
}

fn send(r: RequestBuilder) -> crate::Result<CtpResponse> {
    Ok(r.send().map(CtpResponse::new)?)
}

#[cfg(test)]
mod tests {
    use super::super::region::Region;
    use super::*;

    #[test]
    fn new_client() {
        CtpClient::new(&Region::Europe, "project_key", "client_id", "client_secret");
        CtpClient::new(
            &Region::NorthAmerica,
            "project_key",
            "client_id",
            "client_secret",
        );
    }

    #[test]
    fn new_client_with_customized_url() {
        CtpClient::new(&Region::Europe, "project_key", "client_id", "client_secret")
            .with_api_url("my_api_url");

        CtpClient::new(&Region::Europe, "project_key", "client_id", "client_secret")
            .with_auth_url("my_auth_url");

        CtpClient::new(&Region::Europe, "project_key", "client_id", "client_secret")
            .with_api_url("my_api_url")
            .with_auth_url("my_auth_url");
    }
}
