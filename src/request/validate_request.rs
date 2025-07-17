use asknothingx2_util::{
    api::{
        request::{IntoRequestParts, RequestParts},
        AuthScheme, Method,
    },
    oauth::{AccessToken, ValidateUrl},
};

use crate::{oauth::VALIDATE_URL, types::ValidateToken, Error, APPTYPE};

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
pub async fn validate_token(access_token: &AccessToken) -> Result<ValidateToken, Error> {
    ValidateRequest::new(access_token, &VALIDATE_URL)
        .into_request_parts()
        .send()
        .await
        .map_err(Error::from)?
        .json::<ValidateToken>()
        .await
        .map_err(Error::from)
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

impl IntoRequestParts for ValidateRequest<'_> {
    fn into_request_parts(self) -> RequestParts {
        let mut request = RequestParts::new(Method::GET, self.validate_url.url().clone(), APPTYPE);
        request
            .header_mut()
            .authorization(AuthScheme::custom("OAuth", self.access_token.secret()));

        request
    }
}
