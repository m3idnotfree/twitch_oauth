use asknothingx2_util::api::{mime_type::Application, Method};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client, RequestBuilder, Response,
};

use crate::{
    error,
    types::{GrantType, Token},
    ClientId, ClientSecret, Error, RefreshToken, TokenError, TokenUrl,
};

use super::{IntoRequestBuilder, CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

/// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
#[derive(Debug)]
pub struct RefreshRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    refresh_token: RefreshToken,
    token_url: &'a TokenUrl,
    client: &'a Client,
}

impl<'a> RefreshRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        refresh_token: RefreshToken,
        token_url: &'a TokenUrl,
        client: &'a Client,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            refresh_token,
            token_url,
            client,
        }
    }

    pub async fn send(self) -> Result<Response, Error> {
        let client = self.client.clone();
        self.into_request_builder(&client)?
            .send()
            .await
            .map_err(error::network::request)
    }

    pub async fn json(self) -> Result<Token, Error> {
        let resp = self.send().await?;

        if resp.status().is_success() {
            resp.json::<Token>().await.map_err(error::validation::json)
        } else {
            let error_response: TokenError = resp.json().await.map_err(error::validation::json)?;
            Err(error::oauth::from_token_error(error_response))
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
        .map_err(error::validation::url_encode)?;

        Ok(client
            .request(Method::POST, self.token_url.url().clone())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(form_string))
    }
}
