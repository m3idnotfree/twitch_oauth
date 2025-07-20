use asknothingx2_util::api::{mime_type::Application, Method};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client, Response,
};

use crate::{
    error,
    oauth::TOKEN_URL,
    types::{GrantType, Token},
    AuthorizationCode, ClientId, ClientSecret, Error, RedirectUrl, TokenError,
};

use super::{IntoRequestBuilder, CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

#[derive(Debug)]
pub struct CodeTokenRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
    code: AuthorizationCode,
    redirect_url: &'a RedirectUrl,
    client: &'a Client,
}

impl<'a> CodeTokenRequest<'a> {
    pub fn new(
        client_id: &'a ClientId,
        client_secret: &'a ClientSecret,
        code: AuthorizationCode,
        redirect_url: &'a RedirectUrl,
        client: &'a Client,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            code,
            redirect_url,
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
            .request(Method::POST, TOKEN_URL.url().clone())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(form_string);

        Ok(client)
    }
}
