use asknothingx2_util::{
    api::{form_urlencoded_serialize, APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId, RevocationUrl},
};
use url::Url;

/// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
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
    fn url(&self) -> Url {
        self.revoke_url.url().clone()
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
            ("token", self.access_token.secret()),
        ];

        Some(form_urlencoded_serialize(params))
    }
}
