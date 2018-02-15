use chrono::*;

use hyper::Client;
use hyper::header::{Authorization, Basic, Headers};
use hyper::status::StatusCode;
use serde_json;
use std::fmt;
use std::io::Read;

/// access token
#[derive(Debug, Clone)]
pub struct Token {
    pub bearer_token: Vec<u8>,
    expires_at: DateTime<UTC>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let token = String::from_utf8(self.bearer_token.clone()).map_err(|e| {
            error!("{}", e);
            fmt::Error::default()
        })?;
        write!(
            f,
            "Token: access_token = {}, expires_at = {}",
            token, self.expires_at
        )
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
        debug!(
            "check if now ({}) is valid for expiration date {} with a margin of {}",
            now, self.expires_at, margin
        );
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
pub fn retrieve_token(
    client: &Client,
    auth_url: &str,
    project_key: &str,
    client_id: &str,
    client_secret: &str,
    permissions: &[&str],
) -> ::Result<Token> {
    info!(
        "retrieving a new OAuth token from '{}' for project '{}' with client '{}'",
        auth_url, project_key, client_id
    );
    let mut auth_headers = Headers::new();
    auth_headers.set(Authorization(Basic {
        username: client_id.to_owned(),
        password: Some(client_secret.to_owned()),
    }));

    let scope = permissions
        .iter()
        .map(|&p| format!("{}:{}", p, project_key))
        .collect::<Vec<String>>()
        .join(" ");

    let url = format!(
        "{}/oauth/token?grant_type=client_credentials&scope={}",
        auth_url, scope
    );

    debug!("Trying to retrieve token with url '{}'", url);
    let mut res = client.post(&url).headers(auth_headers).send()?;

    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    if res.status != StatusCode::Ok {
        let err = ::UnexpectedStatus::new("expected OK".to_string(), format!("{:?}", res));
        Err(err)?
    } else {
        debug!("Response from '{}': {}", url, body);
        let token_from_api = serde_json::from_str::<TokenFromApi>(&body)?;
        let bearer_token = String::from("Bearer ") + token_from_api.access_token.as_str();
        Ok(Token::new(
            bearer_token.into_bytes(),
            token_from_api.expires_in,
        ))
    }
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
