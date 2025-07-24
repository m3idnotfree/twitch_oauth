use asknothingx2_util::{
    api::{AuthScheme, IntoRequestBuilder, Method},
    oauth::{AccessToken, ValidateUrl},
};
use reqwest::{header::AUTHORIZATION, Client, RequestBuilder};

use crate::{error, oauth::VALIDATE_URL, types::ValidateToken, Error};

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
pub async fn validate_access_token(
    access_token: &AccessToken,
    client: &Client,
) -> Result<ValidateToken, Error> {
    ValidateRequest::new(access_token, &VALIDATE_URL)
        .into_request_builder(client)?
        .send()
        .await
        .map_err(error::network::request)?
        .json::<ValidateToken>()
        .await
        .map_err(error::response::json)
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

impl IntoRequestBuilder for ValidateRequest<'_> {
    type Error = Error;

    fn into_request_builder(self, client: &Client) -> Result<RequestBuilder, Error> {
        Ok(client
            .request(Method::GET, self.validate_url.url().clone())
            .header(
                AUTHORIZATION,
                AuthScheme::custom("OAuth", self.access_token.secret()).to_header_value()?,
            ))
    }
}
