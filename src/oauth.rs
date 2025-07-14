use asknothingx2_util::{
    api::{request::IntoRequestParts, Response},
    oauth::{
        AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
        RefreshToken, RevocationUrl, TokenUrl,
    },
};
use http_serde::http::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    error,
    types::{GrantType, ResponseType},
    AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest, Error, RefreshRequest,
    RevokeRequest,
};

const BASE_URL: &str = "https://id.twitch.tv/oauth2";
const AUTH: &str = "authorize";
const TOKEN: &str = "token";
const REVOKE: &str = "revoke";

/// OAuth client for Twitch API authentication.
///
/// This client handles all OAuth flows including authorization code,
/// client credentials, and token refresh/revocation.
///
/// # Examples
///
/// ```rust
/// use twitch_oauth_token::TwitchOauth;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let oauth = TwitchOauth::new(
///         "client_id".to_string(),
///         "client_secret".to_string(),
///         Some("http://localhost:3000/callback".to_string())
///     )?;
///     
///     let token = oauth.client_credentials().await?;
///     println!("Got token: {:?}", token);
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct TwitchOauth {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: Option<RedirectUrl>,
    #[cfg(feature = "test")]
    test_user: crate::test_url::TestUrlHold,
    #[cfg(feature = "test")]
    test_app: crate::test_url::TestUrlHold,
}

impl TwitchOauth {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: Option<String>,
    ) -> Result<Self, Error> {
        Ok(Self {
            client_id: ClientId::new(client_id),
            client_secret: ClientSecret::new(client_secret),
            redirect_uri: match redirect_uri {
                Some(uri) => {
                    Some(RedirectUrl::new(uri).map_err(error::oauth::invalid_redirect_url)?)
                }
                None => None,
            },
            #[cfg(feature = "test")]
            test_user: crate::test_url::TestUrlHold::default(),
            #[cfg(feature = "test")]
            test_app: crate::test_url::TestUrlHold::default(),
        })
    }

    pub fn from_credentials(
        client_id: ClientId,
        client_secret: ClientSecret,
        redirect_uri: Option<String>,
    ) -> Result<Self, Error> {
        Ok(Self {
            client_id,
            client_secret,
            redirect_uri: match redirect_uri {
                Some(uri) => {
                    Some(RedirectUrl::new(uri).map_err(error::oauth::invalid_redirect_url)?)
                }
                None => None,
            },
            #[cfg(feature = "test")]
            test_user: crate::test_url::TestUrlHold::default(),
            #[cfg(feature = "test")]
            test_app: crate::test_url::TestUrlHold::default(),
        })
    }

    pub fn set_redirect_uri(mut self, redir_url: String) -> Result<Self, Error> {
        self.redirect_uri =
            Some(RedirectUrl::new(redir_url).map_err(error::oauth::invalid_redirect_url)?);
        Ok(self)
    }

    pub fn get_redirect_uri(&self) -> Option<Url> {
        self.redirect_uri.clone().map(|uri| uri.url().clone())
    }

    pub fn get_auth_url(&self) -> AuthUrl {
        AuthUrl::new(format!("{BASE_URL}/{AUTH}")).unwrap()
    }

    fn get_token_url(&self) -> TokenUrl {
        TokenUrl::new(format!("{BASE_URL}/{TOKEN}")).unwrap()
    }

    fn get_revoke_url(&self) -> RevocationUrl {
        RevocationUrl::new(format!("{BASE_URL}/{REVOKE}")).unwrap()
    }

    fn validate_redirect_uri(&self) -> Result<RedirectUrl, Error> {
        self.redirect_uri
            .clone()
            .ok_or_else(error::oauth::missing_redirect_url)
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub fn authorize_url(&mut self) -> Result<(AuthrozationRequest, CsrfToken), Error> {
        let csrf_token = CsrfToken::new_random();

        let auth_request = AuthrozationRequest::new(
            self.get_auth_url(),
            self.client_id.clone(),
            self.validate_redirect_uri()?,
            ResponseType::Code,
            csrf_token.clone(),
        );

        Ok((auth_request, csrf_token))
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub async fn exchange_code_for_token(
        &self,
        code: AuthorizationCode,
    ) -> Result<Response, Error> {
        CodeTokenRequest::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            code,
            GrantType::AuthorizationCode,
            self.get_token_url(),
            self.validate_redirect_uri()?,
        )
        .into_request_parts()
        .send()
        .await
        .map_err(error::network::request)
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    #[cfg(feature = "oneshot-server")]
    pub async fn exchange_code(
        &mut self,
        code_state: crate::oneshot_server::CodeState,
        csrf_token: CsrfToken,
    ) -> Result<Response, Error> {
        match code_state.state {
            crate::oneshot_server::ServerStatus::Timeout => Err(error::server::timeout()),
            crate::oneshot_server::ServerStatus::Shutdown => Err(error::server::shutdown()),
            crate::oneshot_server::ServerStatus::Recive => {
                let received_csrf = code_state
                    .csrf_token
                    .ok_or_else(error::oauth::missing_csrf_token)?;

                if received_csrf.secret() != csrf_token.secret() {
                    return Err(error::oauth::csrf_token_mismatch());
                }

                let code = code_state
                    .code
                    .ok_or_else(error::oauth::missing_auth_code)?;
                self.exchange_code_for_token(code).await
            }
        }
    }
    /// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
    pub async fn exchange_refresh_token(
        &self,
        refresh_token: RefreshToken,
    ) -> Result<Response, Error> {
        RefreshRequest::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::RefreshToken,
            refresh_token,
            self.get_token_url(),
        )
        .into_request_parts()
        .send()
        .await
        .map_err(error::network::request)
    }

    /// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
    pub async fn revoke_token(&self, access_token: AccessToken) -> Result<Response, Error> {
        RevokeRequest::new(access_token, self.client_id.clone(), self.get_revoke_url())
            .into_request_parts()
            .send()
            .await
            .map_err(error::network::request)
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
    pub async fn client_credentials(&self) -> Result<Response, Error> {
        ClientCredentialsRequest::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::ClientCredentials,
            self.get_token_url(),
        )
        .into_request_parts()
        .send()
        .await
        .map_err(error::network::request)
    }

    #[cfg(feature = "test")]
    pub fn with_url(mut self, port: Option<u16>) -> Self {
        let mut user_url = Url::parse("http://localhost:8080/auth/authorize").unwrap();
        let mut app_url = Url::parse("http://localhost:8080/auth/token").unwrap();
        if let Some(port) = port {
            user_url.set_port(Some(port)).unwrap();
            app_url.set_port(Some(port)).unwrap();
        }
        self.test_user = self.test_user.with_url(user_url.to_string());
        self.test_app = self.test_app.with_url(app_url.to_string());
        self
    }
    /// Getting a user access token
    #[cfg(feature = "test")]
    pub fn user_token(&self, user_id: String) -> crate::test_url::TestAccessToken {
        crate::test_url::TestAccessToken::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::UserToken,
            Some(user_id),
            std::collections::HashSet::new(),
            self.test_auth_url(TestOauthType::User),
        )
    }
    /// Getting an app access token
    #[cfg(feature = "test")]
    pub fn app_token(&self) -> crate::test_url::TestAccessToken {
        crate::test_url::TestAccessToken::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::ClientCredentials,
            None,
            std::collections::HashSet::new(),
            self.test_auth_url(TestOauthType::App),
        )
    }

    #[cfg(feature = "test")]
    fn test_auth_url(&self, kind: TestOauthType) -> AuthUrl {
        match kind {
            TestOauthType::App => {
                if let Some(url) = &self.test_app.get_test_url() {
                    return AuthUrl::new(url.clone()).unwrap();
                }

                AuthUrl::new(format!("{BASE_URL}/{AUTH}")).unwrap()
            }
            TestOauthType::User => {
                if let Some(url) = &self.test_user.get_test_url() {
                    return AuthUrl::new(url.clone()).unwrap();
                }

                AuthUrl::new(format!("{BASE_URL}/{AUTH}")).unwrap()
            }
        }
    }
}

#[cfg(feature = "test")]
enum TestOauthType {
    User,
    App,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenError {
    #[serde(with = "http_serde::status_code")]
    pub status: StatusCode,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OAuth error {}: {}", self.status, self.message)?;
        if let Some(ref error) = self.error {
            write!(f, " ({error})")?;
        }
        Ok(())
    }
}

impl std::error::Error for TokenError {}
