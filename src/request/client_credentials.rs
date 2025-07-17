use asknothingx2_util::{
    api::{
        request::{IntoRequestParts, RequestBody, RequestParts},
        Method, Response,
    },
    oauth::{ClientId, ClientSecret},
};

use crate::{
    error,
    oauth::TOKEN_URL,
    types::{ClientCredentials, GrantType},
    Error, TokenError, APPTYPE,
};

use super::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
#[derive(Debug)]
pub struct ClientCredentialsRequest<'a> {
    client_id: &'a ClientId,
    client_secret: &'a ClientSecret,
}

impl<'a> ClientCredentialsRequest<'a> {
    pub fn new(client_id: &'a ClientId, client_secret: &'a ClientSecret) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }

    pub async fn send(self) -> Result<Response, Error> {
        self.into_request_parts()
            .send()
            .await
            .map_err(error::network::request)
    }

    pub async fn json(self) -> Result<ClientCredentials, Error> {
        let resp = self
            .into_request_parts()
            .send()
            .await
            .map_err(error::network::request)?;

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

impl IntoRequestParts for ClientCredentialsRequest<'_> {
    fn into_request_parts(self) -> RequestParts {
        let mut request = RequestParts::new(Method::POST, TOKEN_URL.url().clone(), APPTYPE);

        request
            .header_mut()
            .accept_json()
            .content_type_formencoded();

        request.body(RequestBody::from_form_pairs([
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            (GRANT_TYPE, GrantType::ClientCredentials.as_str()),
        ]));

        request
    }
}
