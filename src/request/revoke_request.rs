use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ClientId, RevocationUrl},
};
use url::Url;

#[derive(Debug)]
pub struct RevokeRequest {
    access_token: AccessToken,
    client_id: ClientId,
    revoke_url: RevocationUrl,
}

impl RevokeRequest {
    pub fn new(access_token: AccessToken, client_id: ClientId, revoke_url: RevocationUrl) -> Self {
        Self {
            access_token,
            client_id,
            revoke_url,
        }
    }
}

impl APIRequest for RevokeRequest {
    fn urlencoded(&self) -> Option<Vec<u8>> {
        let params = vec![
            ("client_id", self.client_id.as_str()),
            ("token", self.access_token.secret()),
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
        self.revoke_url.url().clone()
    }
}

#[cfg(test)]
mod tests {
    use asknothingx2_util::{
        api::{APIRequest, Method},
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

        assert_eq!(Method::POST, request.method());
        assert_eq!(
            Url::parse("https://id.twitch.tv/oauth2/revoke").unwrap(),
            request.url()
        );
        assert_eq!(2, request.headers().len());
        assert_eq!(Some(expected_body), request.urlencoded());
    }
}
