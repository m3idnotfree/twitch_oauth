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
    client: &'a Client,
    token_url: &'a TokenUrl,
}

impl<'a> ClientCredentialsRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        client: &'a Client,
        token_url: &'a TokenUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            client,
            token_url,
        }
    }

    pub async fn send(self) -> Result<Response<ClientCredentialsResponse>, Error> {
        let client = self.client.clone();
        let resp = self
            .into_request_builder(&client)?
            .send()
            .await
            .map_err(error::network::request)?;

        Ok(Response::new(resp))
    }

    pub async fn json(self) -> Result<ClientCredentials, Error> {
        let resp = self.send().await?;
        resp.client_credentials().await
    }
}

impl IntoRequestBuilder for ClientCredentialsRequest<'_> {
    type Error = Error;

    fn into_request_builder(self, client: &Client) -> Result<RequestBuilder, Error> {
        let form_string = serde_urlencoded::to_string([
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            (GRANT_TYPE, GrantType::ClientCredentials.as_str()),
        ])
        .map_err(error::validation::url_encode)?;

        Ok(client
            .request(Method::POST, self.token_url.url().clone())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(form_string))
    }
}
