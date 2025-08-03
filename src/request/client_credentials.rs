use asknothingx2_util::{
    api::{mime_type::Application, IntoRequestBuilder, Method},
    oauth::{ClientId, ClientSecret, TokenUrl},
};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client, RequestBuilder,
};

use crate::{error, types::GrantType, Error};

use super::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
#[derive(Debug)]
pub struct ClientCredentialsRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    grant_type: GrantType,
    token_url: &'a TokenUrl,
}

impl<'a> ClientCredentialsRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        grant_type: GrantType,
        token_url: &'a TokenUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type,
            token_url,
        }
    }
}

impl IntoRequestBuilder for ClientCredentialsRequest<'_> {
    type Error = Error;

    fn into_request_builder(self, client: &Client) -> Result<RequestBuilder, Error> {
        let form_string = serde_urlencoded::to_string([
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            (GRANT_TYPE, self.grant_type.as_str()),
        ])
        .map_err(error::validation::form_data)?;

        Ok(client
            .request(Method::POST, self.token_url.url().clone())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(form_string))
    }
}
