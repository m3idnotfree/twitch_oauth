use std::ops::Deref;

use asknothingx2_http::Body;
use asknothingx2_util::api::mime_type::Application;
use http::header::{ACCEPT, CONTENT_TYPE};

use crate::{
    AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenUrl,
    request::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE, IntoRequest},
    types::GrantType,
};

#[derive(Debug)]
pub struct ExchangeCodeRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    code: AuthorizationCode,
    redirect_url: &'a RedirectUrl,
    token_url: &'a TokenUrl,
}

impl<'a> ExchangeCodeRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        code: AuthorizationCode,
        redirect_url: &'a RedirectUrl,
        token_url: &'a TokenUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            code,
            redirect_url,
            token_url,
        }
    }
}

impl IntoRequest for ExchangeCodeRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::ExchangeCode;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        Ok(http::Request::post(self.token_url.to_string())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(Body::from(super::form(&[
                (CLIENT_ID, self.client_id.deref()),
                (CLIENT_SECRET, self.client_secret.secret()),
                ("code", self.code.secret()),
                (GRANT_TYPE, GrantType::AuthorizationCode.as_str()),
                ("redirect_uri", self.redirect_url.as_str()),
            ])?))?)
    }
}
