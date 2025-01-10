use asknothingx2_util::{
    api::{api_request, APIRequest, APIResponse, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ValidateUrl},
};
use url::Url;

use crate::{types::ValidateToken, HttpError};

/// https://dev.twitch.tv/docs/authentication/validate-tokens/
pub async fn validate_token(
    access_token: AccessToken,
) -> Result<APIResponse<ValidateToken>, HttpError> {
    let response = api_request(ValidateRequest::new(
        access_token.clone(),
        ValidateUrl::new("https://id.twitch.tv/oauth2/validate".to_string()).unwrap(),
    ))
    .await?;

    Ok(APIResponse::from_response(response).await?)
}

/// https://dev.twitch.tv/docs/authentication/validate-tokens/
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

impl APIRequest for ValidateRequest {
    fn url(&self) -> Url {
        self.validate_url.url().clone()
    }
    fn method(&self) -> Method {
        Method::GET
    }
    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .authorization("OAuth", self.access_token.secret())
            .build()
    }
}
