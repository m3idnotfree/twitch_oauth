use std::ops::Deref;

use http::{header::AUTHORIZATION, HeaderMap, HeaderValue, StatusCode};
use oauth2::AccessToken;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::traits::OauthRequest;

#[derive(Debug)]
pub struct ValidateRequest<'a> {
    pub access_token: &'a AccessToken,
    pub validate_url: &'a ValidateUrl,
}

impl OauthRequest for ValidateRequest<'_> {
    fn headers(&self) -> Option<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.append(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("OAuth {}", self.access_token.secret())).unwrap(),
        );

        Some(headers)
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
    fn url(&self) -> &str {
        self.validate_url.as_str()
    }
}

/// https://docs.rs/oauth2/latest/src/oauth2/types.rs.html#233
#[derive(Clone)]
pub struct ValidateUrl(Url, String);

impl ValidateUrl {
    pub fn new(url: String) -> std::result::Result<Self, url::ParseError> {
        Ok(Self(Url::parse(&url)?, url))
    }
    pub fn from_url(url: Url) -> Self {
        let s = url.to_string();
        Self(url, s)
    }
    pub fn url(&self) -> &Url {
        &self.0
    }
}

impl Deref for ValidateUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.1
    }
}

impl std::fmt::Debug for ValidateUrl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let mut debug_trait_builder = f.debug_tuple(stringify!(ValidateURL));
        debug_trait_builder.field(&self.1);
        debug_trait_builder.finish()
    }
}
impl<'de> serde::Deserialize<'de> for ValidateUrl {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: ::serde::de::Deserializer<'de>,
    {
        struct UrlVisitor;
        impl<'de> serde::de::Visitor<'de> for UrlVisitor {
            type Value = ValidateUrl;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                formatter.write_str(stringify!($name))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                ValidateUrl::new(v.to_string()).map_err(E::custom)
            }
        }
        deserializer.deserialize_str(UrlVisitor {})
    }
}

impl serde::Serialize for ValidateUrl {
    fn serialize<SE>(&self, serializer: SE) -> Result<SE::Ok, SE::Error>
    where
        SE: ::serde::Serializer,
    {
        serializer.serialize_str(&self.1)
    }
}

impl std::hash::Hash for ValidateUrl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&(self.1), state);
    }
}

impl Ord for ValidateUrl {
    fn cmp(&self, other: &ValidateUrl) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for ValidateUrl {
    fn partial_cmp(&self, other: &ValidateUrl) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ValidateUrl {
    fn eq(&self, other: &ValidateUrl) -> bool {
        self.1 == other.1
    }
}

impl Eq for ValidateUrl {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateToken {
    pub client_id: String,
    pub login: String,
    pub scopes: Vec<String>,
    pub user_id: String,
    pub expires_in: u64,
}

pub struct FailValidate {
    pub status: StatusCode,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use http::{header::AUTHORIZATION, HeaderMap, HeaderValue};
    use oauth2::AccessToken;

    use crate::{
        request::{ValidateRequest, ValidateUrl},
        traits::OauthRequest,
    };

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
        assert_eq!("https://id.twitch.tv/oauth2/validate", request.url());
        assert_eq!(None, request.body());
        assert_eq!(Some(expected_headers), request.headers());
    }
}
