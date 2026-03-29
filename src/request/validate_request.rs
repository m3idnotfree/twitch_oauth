use asknothingx2_util::api::{AuthScheme, IntoRequestBuilder, Method};
use reqwest::{Client, RequestBuilder, header::AUTHORIZATION};

use crate::{AccessToken, Error, ValidateUrl, tokens::TokenInfo};

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
pub async fn validate_access_token(
    access_token: &AccessToken,
    client: &Client,
    validate_url: &ValidateUrl,
) -> Result<TokenInfo, Error> {
    let resp = crate::oauth::send(client, ValidateRequest::new(access_token, validate_url)).await?;
    crate::oauth::decode_response(resp).await
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
            .request(Method::GET, self.validate_url.as_str())
            .header(
                AUTHORIZATION,
                AuthScheme::custom("OAuth", self.access_token.secret()).to_header_value()?,
            ))
    }
}
