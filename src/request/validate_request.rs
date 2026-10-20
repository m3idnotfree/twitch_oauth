use std::str::FromStr;

use asknothingx2_http::{Body, ClientService, client::Reqwest};
use asknothingx2_util::api::AuthScheme;
use http::header::AUTHORIZATION;

use crate::{AccessToken, Error, ValidateUrl, request::IntoRequest, tokens::TokenInfo};

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
pub async fn validate_access_token(access_token: &AccessToken) -> Result<TokenInfo, Error> {
    crate::oauth::json(
        ClientService::new(Reqwest::new(crate::default_client())),
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
        crate::oauth::json(ClientService::new(Reqwest::new(client.clone())), self).await
    }
}

impl IntoRequest for ValidateRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::ValidateAccessToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        let authorization =
            AuthScheme::custom("OAuth", self.access_token.secret()).to_header_value()?;

        Ok(http::Request::get(self.validate_url.to_string())
            .header(AUTHORIZATION, authorization)
            .body(Body::empty())?)
    }
}
