use std::ops::Deref;

use asknothingx2_util::api::{Method, mime_type::Application};
use reqwest::{
    Client, RequestBuilder,
    header::{ACCEPT, CONTENT_TYPE},
};

use crate::{
    ClientId, ClientSecret, TokenUrl,
    request::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE, IntoRequestBuilder},
    types::GrantType,
};

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
    const OPERATION: crate::error::Operation = crate::error::Operation::AppAccessToken;
    type Error = serde_urlencoded::ser::Error;

    fn into_request_builder(self, client: &Client) -> Result<RequestBuilder, Self::Error> {
        let form_string = serde_urlencoded::to_string([
            (CLIENT_ID, self.client_id.deref()),
            (CLIENT_SECRET, self.client_secret.secret()),
            (GRANT_TYPE, self.grant_type.as_str()),
        ])?;

        Ok(client
            .request(Method::POST, self.token_url.as_str())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(form_string))
    }
}
