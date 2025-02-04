use asknothingx2_util::{
    api::{form_urlencoded_serialize, APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{ClientId, ClientSecret, TokenUrl},
};
use url::Url;

use crate::types::GrantType;

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
#[derive(Debug)]
pub struct ClientCredentialsRequest {
    client_id: ClientId,
    client_secret: ClientSecret,
    grant_type: GrantType,
    token_url: TokenUrl,
}

impl ClientCredentialsRequest {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        grant_type: GrantType,
        token_url: TokenUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type,
            token_url,
        }
    }
}

impl APIRequest for ClientCredentialsRequest {
    fn url(&self) -> Url {
        self.token_url.url().clone()
    }
    fn method(&self) -> Method {
        Method::POST
    }
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderBuilder::new();
        headers.accept_json().content_type_formencoded();

        headers.build()
    }
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_str()),
        ];

        Some(form_urlencoded_serialize(params))
    }
}
