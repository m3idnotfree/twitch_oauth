use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{ClientId, ClientSecret, TokenUrl},
};
use url::Url;

use crate::types::GrantType;

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
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_str()),
        ];

        Some(Self::form_urlencoded_serializere_pairs(params))
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .accept_json()
            .content_type_formencoded()
            .build()
    }

    fn method(&self) -> Method {
        Method::POST
    }
    fn url(&self) -> Url {
        self.token_url.url().clone()
    }
}
