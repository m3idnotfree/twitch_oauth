use asknothingx2_util::{
    api::{form_urlencoded_serialize, APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{ClientId, ClientSecret, RefreshToken, TokenUrl},
};
use url::Url;

use crate::types::GrantType;

/// https://dev.twitch.tv/docs/authentication/refresh-tokens/
#[derive(Debug)]
pub struct RefreshRequest {
    client_id: ClientId,
    client_secret: ClientSecret,
    grant_type: GrantType,
    refresh_token: RefreshToken,
    token_url: TokenUrl,
}

impl RefreshRequest {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        grant_type: GrantType,
        refresh_token: RefreshToken,
        token_url: TokenUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type,
            refresh_token,
            token_url,
        }
    }
}

impl APIRequest for RefreshRequest {
    fn url(&self) -> Url {
        self.token_url.url().clone()
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .accept_json()
            .content_type_formencoded()
            .build()
    }
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_str()),
            ("refresh_token", self.refresh_token.secret()),
        ];

        Some(form_urlencoded_serialize(params))
    }
}
