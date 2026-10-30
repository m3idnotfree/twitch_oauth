use asknothingx2_http::{Body, ClientService, client::Reqwest};
use http::{HeaderValue, header::AUTHORIZATION};

use crate::{
    AccessToken, Error,
    request::{IntoRequest, VALIDATE_URL},
    tokens::TokenInfo,
};

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
pub async fn validate_access_token(access_token: &AccessToken) -> Result<TokenInfo, Error> {
    crate::client::json(
        ClientService::new(Reqwest::new(crate::HTTP_CLIENT.clone())),
        ValidateRequest::new(access_token),
    )
    .await
}

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
#[derive(Debug)]
pub struct ValidateRequest<'a> {
    access_token: &'a AccessToken,
}

impl<'a> ValidateRequest<'a> {
    pub fn new(access_token: &'a AccessToken) -> Self {
        Self { access_token }
    }
}

impl IntoRequest for ValidateRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::ValidateAccessToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        let mut authorization =
            HeaderValue::from_str(&format!("OAuth {}", self.access_token.secret()))?;
        authorization.set_sensitive(true);

        Ok(http::Request::get(VALIDATE_URL)
            .header(AUTHORIZATION, authorization)
            .body(Body::empty())?)
    }
}
