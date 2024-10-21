use asknothingx2_util::{
    api::APIRequest,
    oauth::{AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenUrl},
};
use url::Url;

use crate::types::GrantType;

#[derive(Debug)]
pub struct CodeTokenRequest<'a> {
    pub client_id: &'a ClientId,
    pub client_secret: &'a ClientSecret,
    pub code: AuthorizationCode,
    pub grant_type: GrantType,
    pub token_url: &'a TokenUrl,
    pub redirect_url: &'a RedirectUrl,
}

impl APIRequest for CodeTokenRequest<'_> {
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("code", self.code.secret()),
            ("grant_type", self.grant_type.as_ref()),
            ("redirect_uri", self.redirect_url.as_str()),
        ];

        Some(Self::form_urlencoded_serializere_pairs(params))
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn url(&self) -> Url {
        self.token_url.url().clone()
    }
}

#[cfg(test)]
mod tests {
    use asknothingx2_util::{
        api::APIRequest,
        oauth::{AuthorizationCode, ClientId, ClientSecret, RedirectUrl, TokenUrl},
    };
    use url::Url;

    use crate::types::GrantType;

    use super::CodeTokenRequest;

    #[test]
    fn codetoken_rquest() {
        let request = CodeTokenRequest {
            client_id: &ClientId::new("test_id".to_string()),
            client_secret: &ClientSecret::new("test_secret".to_string()),
            code: AuthorizationCode::new("authorization_code".to_string()),
            grant_type: GrantType::AuthorizationCode,
            token_url: &TokenUrl::new("https://id.twitch.tv/oauth2/token".to_string()).unwrap(),
            redirect_url: &RedirectUrl::new("http://localhost:3000".to_string()).unwrap(),
        };
        let params = vec![
            ("client_id", "test_id"),
            ("client_secret", "test_secret"),
            ("code", "authorization_code"),
            ("grant_type", "authorization_code"),
            ("redirect_uri", "http://localhost:3000"),
        ];

        let expected_body = url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(params)
            .finish()
            .into_bytes();

        assert_eq!(http::Method::POST, request.method());
        assert_eq!(
            Url::parse("https://id.twitch.tv/oauth2/token").unwrap(),
            request.url()
        );
        assert_eq!(0, request.headers().len());
        assert_eq!(Some(expected_body), request.urlencoded());
    }
}
