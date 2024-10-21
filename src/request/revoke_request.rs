use asknothingx2_util::{
    api::APIRequest,
    oauth::{AccessToken, ClientId, RevocationUrl},
};
use url::Url;

use super::POST_formencoded_header;

#[derive(Debug)]
pub struct RevokeRequest<'a> {
    pub client_id: &'a ClientId,
    pub access_token: &'a AccessToken,
    pub revoke_url: &'a RevocationUrl,
}

impl APIRequest for RevokeRequest<'_> {
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("token", self.access_token.secret()),
        ];

        Some(Self::form_urlencoded_serializere_pairs(params))
    }

    fn headers(&self) -> http::HeaderMap {
        POST_formencoded_header()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn url(&self) -> Url {
        self.revoke_url.url().clone()
    }
}

#[cfg(test)]
mod tests {
    use asknothingx2_util::{
        api::APIRequest,
        oauth::{AccessToken, ClientId, RevocationUrl},
    };
    use url::Url;

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
        assert_eq!(
            Url::parse("https://id.twitch.tv/oauth2/revoke").unwrap(),
            request.url()
        );
        assert_eq!(2, request.headers().len());
        assert_eq!(Some(expected_body), request.urlencoded());
    }
}
