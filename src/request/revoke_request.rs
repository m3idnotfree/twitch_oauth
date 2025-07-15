use asknothingx2_util::api::{
    request::{IntoRequestParts, RequestBody, RequestParts},
    Method,
};

use crate::APPTYPE;

use crate::{AccessToken, ClientId, RevocationUrl};

use super::CLIENT_ID;

/// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
#[derive(Debug)]
pub struct RevokeRequest<'a> {
    access_token: &'a AccessToken,
    client_id: &'a ClientId,
    revoke_url: &'a RevocationUrl,
}

impl<'a> RevokeRequest<'a> {
    pub fn new(
        access_token: &'a AccessToken,
        client_id: &'a ClientId,
        revoke_url: &'a RevocationUrl,
    ) -> Self {
        Self {
            access_token,
            client_id,
            revoke_url,
        }
    }
}

impl IntoRequestParts for RevokeRequest<'_> {
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
