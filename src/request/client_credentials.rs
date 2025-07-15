use asknothingx2_util::{
    api::{
        request::{IntoRequestParts, RequestBody, RequestParts},
        Method,
    },
    oauth::{ClientId, ClientSecret},
};

use crate::{oauth::TOKEN_URL, types::GrantType, APPTYPE};

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
