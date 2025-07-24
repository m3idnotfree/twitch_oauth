use asknothingx2_util::{
    api::{mime_type::Application, IntoRequestBuilder, Method},
    oauth::{AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenUrl},
};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client,
};

use crate::{
    error::{self},
    types::GrantType,
    Error,
};

use super::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

#[derive(Debug)]
pub struct CodeTokenRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    code: AuthorizationCode,
    redirect_url: &'a RedirectUrl,
    token_url: &'a TokenUrl,
}

impl<'a> CodeTokenRequest<'a> {
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

impl IntoRequestBuilder for CodeTokenRequest<'_> {
    type Error = Error;

    fn into_request_builder(self, client: &Client) -> Result<reqwest::RequestBuilder, Self::Error> {
        let form_string = serde_urlencoded::to_string([
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            ("code", self.code.secret()),
            (GRANT_TYPE, GrantType::AuthorizationCode.as_str()),
            ("redirect_uri", self.redirect_url.as_str()),
        ])
        .map_err(error::validation::url_encode)?;

        let client = client
            .request(Method::POST, self.token_url.url().clone())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(form_string);

        Ok(client)
    }
}
