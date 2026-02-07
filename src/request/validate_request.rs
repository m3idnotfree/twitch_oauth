use asknothingx2_util::api::{AuthScheme, IntoRequestBuilder, Method};
use reqwest::{header::AUTHORIZATION, Client, RequestBuilder};

use crate::{error, tokens::TokenInfo, AccessToken, Error, ValidateUrl};

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
pub async fn validate_access_token(
    access_token: &AccessToken,
    client: &Client,
    validate_url: &ValidateUrl,
) -> Result<TokenInfo, Error> {
    let resp = ValidateRequest::new(access_token, validate_url)
        .into_request_builder(client)?
        .send()
        .await
        .map_err(error::network::request)?;

    if !resp.status().is_success() {
        let status = resp.status().as_u16();
        let v = resp.bytes().await?;
        let body = String::from_utf8_lossy(&v).to_string();
        return Err(error::oauth::http_error(status, body));
    }

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
