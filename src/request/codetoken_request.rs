use asknothingx2_util::api::{
    request::{IntoRequestParts, RequestBody, RequestParts},
    Method,
};

use crate::{
    oauth::TOKEN_URL, types::GrantType, AuthorizationCode, ClientId, ClientSecret, RedirectUrl,
    APPTYPE,
};

use super::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

#[derive(Debug)]
pub struct CodeTokenRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    code: &'a AuthorizationCode,
    redirect_url: &'a RedirectUrl,
}

impl<'a> CodeTokenRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        code: &'a AuthorizationCode,
        redirect_url: &'a RedirectUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            code,
            redirect_url,
        }
    }
}

impl IntoRequestParts for CodeTokenRequest<'_> {
    fn into_request_parts(self) -> RequestParts {
        let mut request = RequestParts::new(Method::POST, TOKEN_URL.url().clone(), APPTYPE);

        request
            .header_mut()
            .accept_json()
            .content_type_formencoded();

        request.body(RequestBody::from_form_pairs([
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            ("code", self.code.secret()),
            (GRANT_TYPE, GrantType::AuthorizationCode.as_str()),
            ("redirect_uri", self.redirect_url.as_str()),
        ]));

        request
    }
}
