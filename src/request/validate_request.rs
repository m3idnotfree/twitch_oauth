use asknothingx2_util::{
    api::{
        request::{IntoRequestParts, RequestParts},
        AuthScheme, Method, Response,
    },
    oauth::{AccessToken, ValidateUrl},
};

use crate::{Error, APPTYPE};

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
pub async fn validate_token(access_token: AccessToken) -> Result<Response, Error> {
    ValidateRequest::new(
        access_token.clone(),
        ValidateUrl::new("https://id.twitch.tv/oauth2/validate".to_string()).unwrap(),
    )
    .into_request_parts()
    .send()
    .await
    .map_err(Error::from)
}

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
#[derive(Debug)]
pub struct ValidateRequest {
    access_token: AccessToken,
    validate_url: ValidateUrl,
}

impl ValidateRequest {
    pub fn new(access_token: AccessToken, validate_url: ValidateUrl) -> Self {
        Self {
            access_token,
            validate_url,
        }
    }
}

impl IntoRequestParts for ValidateRequest {
    fn into_request_parts(self) -> RequestParts {
        let mut request = RequestParts::new(Method::GET, self.validate_url.url().clone(), APPTYPE);
        request
            .header_mut()
            .authorization(AuthScheme::custom("OAuth", self.access_token.secret()));

        request
    }
}
