use oauth2::{ClientId, ClientSecret, TokenUrl};

use crate::{traits::OauthRequest, types::GrantType};

#[derive(Debug)]
pub struct ClientCredentialsRequest<'a> {
    pub client_id: &'a ClientId,
    pub client_secret: &'a ClientSecret,
    pub grant_type: GrantType,
    pub token_url: &'a TokenUrl,
}

impl OauthRequest for ClientCredentialsRequest<'_> {
    fn body(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_ref()),
        ];

        Some(Self::urlencoded_serializere_pairs(params))
    }
    fn method(&self) -> http::Method {
        http::Method::POST
    }
    fn url(&self) -> &str {
        self.token_url.as_str()
    }
}
#[cfg(test)]
mod tests {
    use oauth2::{ClientId, ClientSecret, TokenUrl};

    use crate::{traits::OauthRequest, types::GrantType};

    use super::ClientCredentialsRequest;

    #[test]
    fn client_credentials() {
        let request = ClientCredentialsRequest {
            client_id: &ClientId::new("test_id".to_string()),
            client_secret: &ClientSecret::new("test_secret".to_string()),
            grant_type: GrantType::ClientCredentials,
            token_url: &TokenUrl::new("https://id.twitch.tv/oauth2/token".to_string()).unwrap(),
        };

        let params = vec![
            ("client_id", "test_id"),
            ("client_secret", "test_secret"),
            ("grant_type", "client_credentials"),
        ];

        let expected_body = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(params)
            .finish()
            .into_bytes();

        assert_eq!(http::Method::POST, request.method());
        assert_eq!("https://id.twitch.tv/oauth2/token", request.url());
        assert_eq!(None, request.headers());
        assert_eq!(Some(expected_body), request.body());
    }
}
