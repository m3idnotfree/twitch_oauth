use asknothingx2_util::{
    api::{APIRequest, HeaderBuilder, HeaderMap, Method},
    oauth::{AccessToken, ValidateUrl},
};
use url::Url;

#[derive(Debug)]
pub struct ValidateRequest<'a> {
    pub access_token: &'a AccessToken,
    pub validate_url: &'a ValidateUrl,
}

impl APIRequest for ValidateRequest<'_> {
    fn headers(&self) -> HeaderMap {
        HeaderBuilder::new()
            .authorization("OAuth", self.access_token.secret())
            .build()
    }

    fn method(&self) -> Method {
        Method::GET
    }
    fn url(&self) -> Url {
        self.validate_url.url().clone()
    }
}

#[cfg(test)]
mod tests {
    use asknothingx2_util::{
        api::{APIRequest, HeaderBuilder, Method},
        oauth::{AccessToken, ValidateUrl},
    };
    use url::Url;

    use crate::request::ValidateRequest;

    #[test]
    fn validate_request() {
        let request = ValidateRequest {
            access_token: &AccessToken::new("ue85uei4ui".to_string()),
            validate_url: &ValidateUrl::new("https://id.twitch.tv/oauth2/validate".to_string())
                .unwrap(),
        };

        let expected_headers = HeaderBuilder::new()
            .authorization("OAuth", "ue85uei4ui")
            .build();

        assert_eq!(Method::GET, request.method());
        assert_eq!(
            Url::parse("https://id.twitch.tv/oauth2/validate").unwrap(),
            request.url()
        );
        assert_eq!(expected_headers, request.headers());
    }
}
