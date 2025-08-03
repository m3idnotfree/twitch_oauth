use asknothingx2_util::{
    api::{mime_type::Application, IntoRequestBuilder, Method},
    oauth::{ClientId, ClientSecret, RefreshToken, TokenUrl},
};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client, RequestBuilder,
};

use crate::{error, types::GrantType, Error};

use super::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

/// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
#[derive(Debug)]
pub struct RefreshRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    refresh_token: RefreshToken,
    token_url: &'a TokenUrl,
}

impl<'a> RefreshRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        refresh_token: RefreshToken,
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

impl IntoRequestBuilder for RefreshRequest<'_> {
    type Error = Error;

    fn into_request_builder(self, client: &Client) -> Result<RequestBuilder, Error> {
        let form_string = serde_urlencoded::to_string([
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            (GRANT_TYPE, GrantType::RefreshToken.as_str()),
            ("refresh_token", self.refresh_token.secret()),
        ])
        .map_err(error::validation::form_data)?;

        Ok(client
            .request(Method::POST, self.token_url.url().clone())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(form_string))
    }
}
