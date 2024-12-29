use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ValidateUrl},
};
use url::Url;

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
