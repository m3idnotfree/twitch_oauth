use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId, RevocationUrl},
};
use url::Url;

#[derive(Debug)]
pub struct RevokeRequest {
    access_token: AccessToken,
    client_id: ClientId,
    revoke_url: RevocationUrl,
}

impl RevokeRequest {
    pub fn new(access_token: AccessToken, client_id: ClientId, revoke_url: RevocationUrl) -> Self {
        Self {
            access_token,
            client_id,
            revoke_url,
        }
    }
}

impl APIRequest for RevokeRequest {
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("token", self.access_token.secret()),
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
        self.revoke_url.url().clone()
    }
}
