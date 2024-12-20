use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenUrl},
};
use url::Url;

use crate::types::GrantType;

#[derive(Debug)]
pub struct CodeTokenRequest {
    client_id: ClientId,
    client_secret: ClientSecret,
    code: AuthorizationCode,
    grant_type: GrantType,
    token_url: TokenUrl,
    redirect_url: RedirectUrl,
}

impl CodeTokenRequest {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        code: AuthorizationCode,
        grant_type: GrantType,
        token_url: TokenUrl,
        redirect_url: RedirectUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            code,
            grant_type,
            token_url,
            redirect_url,
        }
    }
}

impl APIRequest for CodeTokenRequest {
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("code", self.code.secret()),
            ("grant_type", self.grant_type.as_ref()),
            ("redirect_uri", self.redirect_url.as_str()),
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
