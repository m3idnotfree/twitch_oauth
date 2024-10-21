use asknothingx2_util::oauth::{AuthUrl, ClientId, CsrfToken, RedirectUrl, Scope};
use url::Url;

use crate::types::ResponseType;

pub struct AuthrozationRequest<'a> {
    pub auth_url: &'a AuthUrl,
    pub client_id: &'a ClientId,
    pub redirect_url: &'a RedirectUrl,
    pub response_type: ResponseType,
    pub scopes: Vec<Scope>,
    pub state: CsrfToken,
}

impl<'a> AuthrozationRequest<'a> {
    pub fn add_scope(mut self, scope: &str) -> Self {
        self.scopes.push(Scope::new(scope.to_string()));
        self
    }

    pub fn add_scopes<'de, I>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = &'de str>,
    {
        self.scopes.extend(
            scopes
                .into_iter()
                .map(|x| Scope::new(x.to_string()))
                .collect::<Vec<Scope>>(),
        );
        self
    }

    pub fn url(self) -> Url {
        let scopes = self
            .scopes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        let url = {
            let mut pairs = vec![
                ("client_id", self.client_id.as_str()),
                ("redirect_uri", self.redirect_url.as_str()),
                ("response_type", self.response_type.as_ref()),
                ("state", self.state.secret()),
            ];

            if !scopes.is_empty() {
                pairs.push(("scope", &scopes));
            }

            let mut url: Url = self.auth_url.url().to_owned();

            url.query_pairs_mut().extend_pairs(pairs);

            url
        };

        url
    }
}

#[cfg(test)]
mod tests {
    use asknothingx2_util::oauth::{AuthUrl, ClientId, CsrfToken, RedirectUrl};
    use url::Url;

    use crate::types::ResponseType;

    use super::AuthrozationRequest;

    #[test]
    fn authorize_request() {
        let csrf_token = CsrfToken::new_random();
        let request = AuthrozationRequest {
            auth_url: &AuthUrl::new("https://id.twitch.tv/oauth2/authorize".to_string()).unwrap(),
            client_id: &ClientId::new("test_id".to_string()),
            redirect_url: &RedirectUrl::new("http://localhost:3000".to_string()).unwrap(),
            response_type: ResponseType::Token,
            scopes: Vec::new(),
            state: csrf_token.clone(),
        };

        let request = request.add_scope("chat:read");
        let request = request.add_scopes(vec!["addscopes:vec"]);

        let expected_scopes = "chat:read addscopes:vec";
        let expected_pairs = vec![
            ("client_id", "test_id"),
            ("redirect_uri", "http://localhost:3000"),
            ("response_type", ResponseType::Token.as_ref()),
            ("state", csrf_token.secret()),
            ("scope", expected_scopes),
        ];
        let mut expected_url = Url::parse("https://id.twitch.tv/oauth2/authorize").unwrap();

        expected_url.query_pairs_mut().extend_pairs(expected_pairs);

        let auth_url = request.url();

        assert_eq!(expected_url, auth_url);
    }
}
