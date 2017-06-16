use chrono::*;

use hyper;
use hyper::client::Client;
use hyper::client::HttpConnector;
use hyper::client::Request;
use hyper::Method;
use hyper::Uri;
use futures::future;
use futures::future::Future;
use hyper::Body;
use hyper::header::{Headers, Authorization, Basic};
use hyper::StatusCode;
use serde_json;
use std::fmt;
use std::io::Read;
use errors;
use errors::Error;
use futures::Stream;

/// access token
#[derive(Debug, Clone)]
pub struct Token {
    pub bearer_token: Vec<u8>,
    expires_at: DateTime<UTC>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token = String::from_utf8(self.bearer_token.clone())
            .map_err(|e| {
                         error!("{}", e);
                         fmt::Error::default()
                     })?;
        write!(f,
               "Token: access_token = {}, expires_at = {}",
               token,
               self.expires_at)
    }
}

impl Token {
    pub fn new(bearer_token: Vec<u8>, expires_in_s: i64) -> Token {
        let duration = Duration::seconds(expires_in_s);
        Token {
            bearer_token: bearer_token,
            expires_at: UTC::now() + duration,
        }
    }

    pub fn is_valid_with_margin(&self, now: DateTime<UTC>, margin: Duration) -> bool {
        debug!("check if now ({}) is valid for expiration date {} with a margin of {}",
               now,
               self.expires_at,
               margin);
        now + margin < self.expires_at
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_with_margin(UTC::now(), Duration::seconds(30))
    }
}

// internal data structure to read response from API
#[derive(Deserialize, Debug)]
struct TokenFromApi {
    access_token: String,
    expires_in: i64,
}

/// retrieve an [OAuth token](http://dev.commercetools.com/http-api-authorization.html)
/// for the commercetools API
pub fn retrieve_token(client: &Client<HttpConnector, Body>,
                      auth_url: &str,
                      project_key: &str,
                      client_id: &str,
                      client_secret: &str,
                      permissions: &[&str])
                      -> Box<Future<Item=Token, Error=Error>> {

    info!("retrieving a new OAuth token from '{}' for project '{}' with client '{}'",
          auth_url,
          project_key,
          client_id);

    let scope = permissions
        .iter()
        .map(|&p| format!("{}:{}", p, project_key))
        .collect::<Vec<String>>()
        .join(" ");

    let url = format!("{}/oauth/token?grant_type=client_credentials&scope={}",
                      auth_url,
                      scope);

    let parsed_url = match url.parse::<Uri>() {
        Err(e) => return Box::new(future::err(e.into())),
        Ok(url) => url,
    };

    debug!("Trying to retrieve token with url '{:?}'", &url);
    let mut request = Request::new(Method::Post, parsed_url);
    request.headers_mut().set(Authorization(Basic {
                                       username: client_id.to_owned(),
                                       password: Some(client_secret.to_owned()),
                                   }));

    let result = client.request(request)
        .then(move |res| {
            match res {
                Ok(res) => {
                    if res.status() != StatusCode::Ok {
                        let err: Error = ::ErrorKind::UnexpectedStatus("expected OK".to_string(), format!("{:?}", res)).into();
                        Box::new(future::err(err))
                    } else {
                        let r: Box<Future<Item=Token, Error=Error>> =
                            Box::new(res.body().concat().then(move |res| {
                               match res {
                                    Ok(body) => {
                                        debug!("Response from '{}': {:?}", &url, body);
                                        let token_from_api = serde_json::from_slice::<TokenFromApi>(&body)?;
                                        let bearer_token = String::from("Bearer ") + token_from_api.access_token.as_str();
                                        Ok(Token::new(bearer_token.into_bytes(), token_from_api.expires_in))
                                    },
                                    Err(e) => Err(e.into()),
                                }
                            }));
                        r
                    }
                },
                Err(e) => Box::new(future::err(e.into())),
            }
        });
    Box::new(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_is_valid_before_expiration_date() {
        let token = Token::new(vec![], 60);
        assert!(token.is_valid());
    }

    #[test]
    fn token_is_not_valid_after_expiration_date() {
        let token = Token::new(vec![], 60);
        let now = UTC::now() + Duration::minutes(2);
        assert!(!token.is_valid_with_margin(now, Duration::seconds(0)));
    }

    #[test]
    fn token_is_not_valid_in_margin() {
        let token = Token::new(vec![], 60);
        let now = UTC::now() + Duration::seconds(50);
        assert!(!token.is_valid_with_margin(now, Duration::seconds(20)));
    }
}
