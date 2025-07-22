use asknothingx2_util::{
    api::{mime_type::Application, Method},
    oauth::{AccessToken, ClientId, RevocationUrl},
};
use reqwest::{
    header::{ACCEPT, CONTENT_TYPE},
    Client, RequestBuilder,
};

use crate::{
    error,
    response::{NoContentResponse, Response},
    Error,
};

use super::{IntoRequestBuilder, CLIENT_ID};

/// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
#[derive(Debug)]
pub struct RevokeRequest<'a> {
    access_token: &'a AccessToken,
    client_id: &'a ClientId,
    revoke_url: &'a RevocationUrl,
    client: &'a Client,
}

impl<'a> RevokeRequest<'a> {
    pub fn new(
        access_token: &'a AccessToken,
        client_id: &'a ClientId,
        revoke_url: &'a RevocationUrl,
        client: &'a Client,
    ) -> Self {
        Self {
            access_token,
            client_id,
            revoke_url,
            client,
        }
    }

    pub async fn send(self) -> Result<Response<NoContentResponse>, Error> {
        let client = self.client.clone();
        let resp = self
            .into_request_builder(&client)?
            .send()
            .await
            .map_err(error::network::request)?;

        Ok(Response::new(resp))
    }
}

impl IntoRequestBuilder for RevokeRequest<'_> {
    type Error = Error;

    fn into_request_builder(self, client: &Client) -> Result<RequestBuilder, Self::Error> {
        let form_string = serde_urlencoded::to_string([
            (CLIENT_ID, self.client_id.as_str()),
            ("token", self.access_token.secret()),
        ])
        .map_err(error::validation::url_encode)?;

        Ok(client
            .request(Method::POST, self.revoke_url.url().clone())
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(form_string))
    }
}
