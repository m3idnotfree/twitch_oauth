use asknothingx2_util::{
    api::{
        request::{IntoRequestParts, RequestBody, RequestParts},
        Method,
    },
    oauth::{AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenUrl},
};

use crate::{types::GrantType, APPTYPE};

use super::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

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

impl IntoRequestParts for CodeTokenRequest {
    fn into_request_parts(self) -> RequestParts {
        let mut request = RequestParts::new(Method::POST, self.token_url.url().clone(), APPTYPE);

        request
            .header_mut()
            .accept_json()
            .content_type_formencoded();

        request.body(RequestBody::from_form_pairs([
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            ("code", self.code.secret()),
            (GRANT_TYPE, self.grant_type.as_str()),
            ("redirect_uri", self.redirect_url.as_str()),
        ]));

        request
    }
}
