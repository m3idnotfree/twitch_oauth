use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{ClientId, ClientSecret, TokenUrl},
};
use url::Url;

use crate::types::GrantType;

#[derive(Debug)]
pub struct ClientCredentialsRequest<'a> {
    pub client_id: &'a ClientId,
    pub client_secret: &'a ClientSecret,
    pub grant_type: GrantType,
    pub token_url: &'a TokenUrl,
}

impl APIRequest for ClientCredentialsRequest<'_> {
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.secret()),
            ("grant_type", self.grant_type.as_ref()),
        ];

        Some(Self::form_urlencoded_serializere_pairs(params))
    }

    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .accept_json()
            .content_type_formencoded()
            .build()
    }

    fn method(&self) -> Method {
        Method::POST
    }
    fn url(&self) -> Url {
        self.token_url.url().clone()
    }
}
#[cfg(test)]
mod tests {
    use asknothingx2_util::{
        api::{APIRequest, Method},
        oauth::{ClientId, ClientSecret, TokenUrl},
    };
    use url::Url;

    use crate::types::GrantType;

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

        assert_eq!(Method::POST, request.method());
        assert_eq!(
            Url::parse("https://id.twitch.tv/oauth2/token").unwrap(),
            request.url()
        );
        assert_eq!(2, request.headers().len());
        assert_eq!(Some(expected_body), request.urlencoded());
    }
}
