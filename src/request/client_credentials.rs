use asknothingx2_util::{
    api::{mime_type::Application, Method},
    oauth::{ClientId, ClientSecret},
};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client, RequestBuilder, Response,
};

use crate::{
    error::{self, TokenError},
    oauth::TOKEN_URL,
    types::{ClientCredentials, GrantType},
    Error,
};

use super::{IntoRequestBuilder, CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
#[derive(Debug)]
pub struct ClientCredentialsRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    client: &'a Client,
}

impl<'a> ClientCredentialsRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        client: &'a Client,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            client,
        }
    }

    pub async fn send(self) -> Result<Response, crate::Error> {
        let client = self.client.clone();
        self.into_request_builder(&client)?
            .send()
            .await
            .map_err(error::network::request)
    }

    pub async fn json(self) -> Result<ClientCredentials, Error> {
        let resp = self.send().await?;

        if resp.status().is_success() {
            resp.json::<ClientCredentials>()
                .await
                .map_err(error::validation::json)
        } else {
            let error_response: TokenError = resp.json().await.map_err(error::validation::json)?;
            Err(error::oauth::from_token_error(error_response))
        }
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
            .request(Method::POST, TOKEN_URL.url().clone())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(form_string))
    }
}
