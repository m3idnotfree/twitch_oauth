use std::str::FromStr;

use asknothingx2_util::api::{AuthScheme, IntoRequestBuilder, Method};
use reqwest::{Client, RequestBuilder, header::AUTHORIZATION};

use crate::{AccessToken, Error, ValidateUrl, tokens::TokenInfo};

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
pub async fn validate_access_token(access_token: &AccessToken) -> Result<TokenInfo, Error> {
    crate::oauth::json(
        crate::client::get(),
        ValidateRequest::new(
            access_token,
            &ValidateUrl::from_str(crate::oauth::VALIDATE_URL).unwrap(),
        ),
    )
    .await
}

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
#[derive(Debug)]
pub struct ValidateRequest<'a> {
    access_token: &'a AccessToken,
    validate_url: &'a ValidateUrl,
}

impl<'a> ValidateRequest<'a> {
    pub fn new(access_token: &'a AccessToken, validate_url: &'a ValidateUrl) -> Self {
        Self {
            access_token,
            validate_url,
        }
    }
}

#[allow(dead_code)]
#[cfg(feature = "test")]
impl<'a> ValidateRequest<'a> {
    pub fn with_validate_url(mut self, url: &'a ValidateUrl) -> Self {
        self.validate_url = url;
        self
    }

    pub async fn send(self, client: &reqwest::Client) -> Result<TokenInfo, Error> {
        crate::oauth::json(client, self).await
    }
}

impl IntoRequestBuilder for ValidateRequest<'_> {
    type Error = Error;

    fn into_request_builder(self, client: &Client) -> Result<RequestBuilder, Error> {
        Ok(client
            .request(Method::GET, self.validate_url.as_str())
            .header(
                AUTHORIZATION,
                AuthScheme::custom("OAuth", self.access_token.secret()).to_header_value()?,
            ))
    }
}
