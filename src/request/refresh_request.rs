use asknothingx2_util::api::{
    request::{IntoRequestParts, RequestBody, RequestParts},
    Method,
};

use crate::{types::GrantType, ClientId, ClientSecret, RefreshToken, TokenUrl, APPTYPE};

use super::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

/// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
#[derive(Debug)]
pub struct RefreshRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    refresh_token: &'a RefreshToken,
    token_url: &'a TokenUrl,
}

impl<'a> RefreshRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        refresh_token: &'a RefreshToken,
        token_url: &'a TokenUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            refresh_token,
            token_url,
        }
    }
}

impl IntoRequestParts for RefreshRequest<'_> {
    fn into_request_parts(self) -> RequestParts {
        let mut request = RequestParts::new(Method::POST, self.token_url.url().clone(), APPTYPE);
        request
            .header_mut()
            .accept_json()
            .content_type_formencoded();

        request.body(RequestBody::from_form_pairs(vec![
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            (GRANT_TYPE, GrantType::RefreshToken.as_str()),
            ("refresh_token", self.refresh_token.secret()),
        ]));

        request
    }
}
