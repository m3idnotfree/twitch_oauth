use asknothingx2_util::oauth::{AuthUrl, ClientId, CsrfToken, RedirectUrl};
use url::Url;

use crate::{
    scopes::{ScopeBuilder, Scopes},
    types::ResponseType,
};

pub struct AuthrozationRequest<'a> {
    pub auth_url: &'a AuthUrl,
    pub client_id: &'a ClientId,
    pub redirect_url: &'a RedirectUrl,
    pub response_type: ResponseType,
    pub scopes: ScopeBuilder,
    pub state: CsrfToken,
}

impl<'a> AuthrozationRequest<'a> {
    pub fn add_scope(mut self, scope: Scopes) -> Self {
        self.scopes.add_scope(scope);
        self
    }

    pub fn add_scopes<I>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = Scopes>,
    {
        self.scopes.add_scopes(scopes);
        self
    }

    pub fn url(self) -> Url {
        let scopes = self.scopes.build();

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

    use crate::{
        scopes::{ScopeBuilder, Scopes},
        types::ResponseType,
    };

    use super::AuthrozationRequest;

    #[test]
    fn authorize_request() {
        let csrf_token = CsrfToken::new_random();
        let request = AuthrozationRequest {
            auth_url: &AuthUrl::new("https://id.twitch.tv/oauth2/authorize".to_string()).unwrap(),
            client_id: &ClientId::new("test_id".to_string()),
            redirect_url: &RedirectUrl::new("http://localhost:3000".to_string()).unwrap(),
            response_type: ResponseType::Token,
            scopes: ScopeBuilder::default(),
            state: csrf_token.clone(),
        };

        let request = request.add_scope(Scopes::ChatRead);
        let request = request
            .add_scopes([
                Scopes::ChannelManageSchedule,
                Scopes::ModeratorManageAutomod,
            ])
            .add_scope(Scopes::UserBot);

        let expected_scopes = "chat:read channel:manage:schedule moderator:manage:automod user:bot";
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
