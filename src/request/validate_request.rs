use asknothingx2_util::{
    api::APIRequest,
    oauth::{AccessToken, ValidateUrl},
};
use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
use url::Url;

#[derive(Debug)]
pub struct ValidateRequest<'a> {
    pub access_token: &'a AccessToken,
    pub validate_url: &'a ValidateUrl,
}

impl APIRequest for ValidateRequest<'_> {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.append(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("OAuth {}", self.access_token.secret())).unwrap(),
        );

        headers
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
    fn url(&self) -> Url {
        self.validate_url.url().clone()
    }
}

#[cfg(test)]
mod tests {
    use asknothingx2_util::{
        api::APIRequest,
        oauth::{AccessToken, ValidateUrl},
    };
    use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
    use url::Url;

    use crate::request::ValidateRequest;

    #[test]
    fn validate_request() {
        let request = ValidateRequest {
            access_token: &AccessToken::new("ue85uei4ui".to_string()),
            validate_url: &ValidateUrl::new("https://id.twitch.tv/oauth2/validate".to_string())
                .unwrap(),
        };

        let mut expected_headers = HeaderMap::new();
        expected_headers.append(
            AUTHORIZATION,
            HeaderValue::from_str("OAuth ue85uei4ui").unwrap(),
        );

        assert_eq!(http::Method::GET, request.method());
        assert_eq!(
            Url::parse("https://id.twitch.tv/oauth2/validate").unwrap(),
            request.url()
        );
        assert_eq!(expected_headers, request.headers());
    }
}
