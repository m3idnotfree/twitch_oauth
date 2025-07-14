use asknothingx2_util::{
    api::{
        request::{IntoRequestParts, RequestBody, RequestParts},
        Method,
    },
    oauth::{AccessToken, ClientId, RevocationUrl},
};

use crate::APPTYPE;

use super::CLIENT_ID;

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

impl IntoRequestParts for RevokeRequest {
    fn into_request_parts(self) -> RequestParts {
        let mut request = RequestParts::new(Method::POST, self.revoke_url.url().clone(), APPTYPE);
        request
            .header_mut()
            .accept_json()
            .content_type_formencoded();

        request.body(RequestBody::from_form_pairs([
            (CLIENT_ID, self.client_id.as_str()),
            ("token", self.access_token.secret()),
        ]));

        request
    }
}
