use http::HeaderMap;

pub trait OauthRequest {
    fn method(&self) -> http::Method;
    fn url(&self) -> &str;
    fn headers(&self) -> Option<HeaderMap> {
        None
    }
    fn body(&self) -> Option<Vec<u8>> {
        None
    }
    fn urlencoded_serializere_pairs(params: Vec<(&str, &str)>) -> Vec<u8> {
        url::form_urlencoded::Serializer::new(String::new())
            .extend_pairs(params)
            .finish()
            .into_bytes()
    }
}
