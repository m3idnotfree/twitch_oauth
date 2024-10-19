use oauth2::{AccessToken, ClientId, ClientSecret, RefreshToken, TokenUrl};

use crate::{traits::OauthRequest, types::GrantType};

#[derive(Debug)]
pub struct RefreshRequest<'a> {
    pub client_id: &'a ClientId,
    pub client_secret: &'a ClientSecret,
    pub grant_type: GrantType,
    pub refresh_token: &'a RefreshToken,
    pub token_url: &'a TokenUrl,
}

impl OauthRequest for RefreshRequest<'_> {
    fn body(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_ref()),
            ("refresh_token", self.refresh_token.secret()),
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

pub struct SucessRefresh {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: RefreshToken,
    pub scope: Vec<String>,
}

#[cfg(test)]
mod tests {

    use oauth2::{ClientId, ClientSecret, RefreshToken, TokenUrl};

    use crate::{request::RefreshRequest, traits::OauthRequest, types::GrantType};

    #[test]
    fn refresh_request() {
        let request = RefreshRequest {
            client_id: &ClientId::new("test_id".to_string()),
            client_secret: &ClientSecret::new("test_secret".to_string()),
            grant_type: GrantType::RefreshToken,
            refresh_token: &RefreshToken::new("refres88efi".to_string()),
            token_url: &TokenUrl::new("https://id.twitch.tv/oauth2/token".to_string()).unwrap(),
        };
        let params = vec![
            ("client_id", "test_id"),
            ("client_secret", "test_secret"),
            ("grant_type", "refresh_token"),
            ("refresh_token", "refres88efi"),
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
