use asknothingx2_util::{
    api::{
        request::{IntoRequestParts, RequestBody, RequestParts},
        Method,
    },
    oauth::{ClientId, ClientSecret, TokenUrl},
};

use crate::{types::GrantType, APPTYPE};

use super::{CLIENT_ID, CLIENT_SECRET, GRANT_TYPE};

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
#[derive(Debug)]
pub struct ClientCredentialsRequest {
    client_id: ClientId,
    client_secret: ClientSecret,
    grant_type: GrantType,
    token_url: TokenUrl,
}

impl ClientCredentialsRequest {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        grant_type: GrantType,
        token_url: TokenUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type,
            token_url,
        }
    }
}

impl IntoRequestParts for ClientCredentialsRequest {
    fn into_request_parts(self) -> RequestParts {
        let mut request = RequestParts::new(Method::POST, self.token_url.url().clone(), APPTYPE);

        request
            .header_mut()
            .accept_json()
            .content_type_formencoded();

        request.body(RequestBody::from_form_pairs([
            (CLIENT_ID, self.client_id.as_str()),
            (CLIENT_SECRET, self.client_secret.secret()),
            (GRANT_TYPE, self.grant_type.as_str()),
        ]));

        request
    }
}
