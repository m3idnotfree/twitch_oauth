use asknothingx2_util::{
    api::{
        request::{IntoRequestParts, RequestBody, RequestParts},
        Method,
    },
    oauth::{ClientId, ClientSecret, RefreshToken, TokenUrl},
};

use crate::{types::GrantType, APPTYPE};

use super::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

/// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
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

impl IntoRequestParts for RefreshRequest {
    fn into_request_parts(self) -> RequestParts {
        let mut request = RequestParts::new(Method::POST, self.token_url.url().clone(), APPTYPE);
        request
            .header_mut()
            .accept_json()
            .content_type_formencoded();

        request.body(RequestBody::from_form_pairs(vec![
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            (GRANT_TYPE, self.grant_type.as_str()),
            ("refresh_token", self.refresh_token.secret()),
        ]));

        request
    }
}
