use oauth2::{AccessToken, ClientId, RevocationUrl};

use crate::traits::OauthRequest;

#[derive(Debug)]
pub struct RevokeRequest<'a> {
    pub client_id: &'a ClientId,
    pub access_token: &'a AccessToken,
    pub revoke_url: &'a RevocationUrl,
}

impl OauthRequest for RevokeRequest<'_> {
    fn body(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("token", self.access_token.secret()),
        ];

        Some(Self::urlencoded_serializere_pairs(params))
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }
    fn url(&self) -> &str {
        self.revoke_url.as_str()
    }
}

#[cfg(test)]
mod tests {
    use oauth2::{AccessToken, ClientId, RevocationUrl};

    use crate::traits::OauthRequest;

    use super::RevokeRequest;

    #[test]
    fn revoke_request() {
        let request = RevokeRequest {
            client_id: &ClientId::new("test_id".to_string()),
            access_token: &AccessToken::new("ue85uei4ui".to_string()),
            revoke_url: &RevocationUrl::new("https://id.twitch.tv/oauth2/revoke".to_string())
                .unwrap(),
        };

        let params = vec![("client_id", "test_id"), ("token", "ue85uei4ui")];

        let expected_body = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(params)
            .finish()
            .into_bytes();

        assert_eq!(http::Method::POST, request.method());
        assert_eq!("https://id.twitch.tv/oauth2/revoke", request.url());
        assert_eq!(None, request.headers());
        assert_eq!(Some(expected_body), request.body());
    }
}
