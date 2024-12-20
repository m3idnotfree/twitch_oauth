use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{ClientId, ClientSecret, RefreshToken, TokenUrl},
};
use url::Url;

use crate::types::GrantType;

#[derive(Debug)]
pub struct RefreshRequest {
    client_id: ClientId,
    client_secret: ClientSecret,
    grant_type: GrantType,
    refresh_token: RefreshToken,
    token_url: TokenUrl,
}

impl RefreshRequest {
    pub fn new(
        client_id: ClientId,
        client_secret: ClientSecret,
        grant_type: GrantType,
        refresh_token: RefreshToken,
        token_url: TokenUrl,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            grant_type,
            refresh_token,
            token_url,
        }
    }
}

impl APIRequest for RefreshRequest {
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_ref()),
            ("refresh_token", self.refresh_token.secret()),
        ];

        Some(Self::form_urlencoded_serializere_pairs(params))
    }

    fn method(&self) -> Method {
        Method::POST
    }
    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .accept_json()
            .content_type_formencoded()
            .build()
    }

    fn url(&self) -> Url {
        self.token_url.url().clone()
    }
}

#[cfg(test)]
mod tests {

    use asknothingx2_util::{
        api::{APIRequest, Method},
        oauth::{ClientId, ClientSecret, RefreshToken, TokenUrl},
    };
    use url::Url;

    use crate::{request::RefreshRequest, types::GrantType};

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

        assert_eq!(Method::POST, request.method());
        assert_eq!(
            Url::parse("https://id.twitch.tv/oauth2/token").unwrap(),
            request.url()
        );
        assert_eq!(2, request.headers().len());
        assert_eq!(Some(expected_body), request.urlencoded());
    }
}
