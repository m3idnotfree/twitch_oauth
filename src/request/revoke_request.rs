use std::ops::Deref;

use asknothingx2_http::Body;
use asknothingx2_util::api::mime_type::Application;
use http::header::{ACCEPT, CONTENT_TYPE};

use crate::{
    AccessToken, ClientId, RevocationUrl,
    request::{CLIENT_ID, IntoRequest},
};

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

impl IntoRequest for RevokeRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::RevokeAccessToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        Ok(http::Request::post(self.revoke_url.to_string())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(Body::from(super::form(&[
                (CLIENT_ID, self.client_id.deref()),
                ("token", self.access_token.secret()),
            ])?))?)
    }
}
