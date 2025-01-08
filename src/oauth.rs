use std::{fmt, marker::PhantomData};

use asknothingx2_util::{
    api::api_request,
    oauth::{
        AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
        RefreshToken, RevocationUrl, TokenUrl,
    },
};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::{
    types::{ClientCredentials, GrantType, ResponseType, Token},
    AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest, Error, RefreshRequest,
    RevokeRequest,
};

const BASE_URL: &str = "https://id.twitch.tv/oauth2";
const AUTH: &str = "authorize";
const TOKEN: &str = "token";
const REVOKE: &str = "revoke";

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
    ) -> crate::Result<Self> {
        Ok(Self {
            client_id: ClientId::new(client_id),
            client_secret: ClientSecret::new(client_secret),
            redirect_uri: match redirect_uri {
                Some(uri) => Some(RedirectUrl::new(uri)?),
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
    ) -> crate::Result<Self> {
        Ok(Self {
            client_id,
            client_secret,
            redirect_uri: match redirect_uri {
                Some(uri) => Some(RedirectUrl::new(uri)?),
                None => None,
            },
            #[cfg(feature = "test")]
            test_user: crate::test_url::TestUrlHold::default(),
            #[cfg(feature = "test")]
            test_app: crate::test_url::TestUrlHold::default(),
        })
    }

    pub fn set_redirect_uri(mut self, redir_url: String) -> crate::Result<Self> {
        self.redirect_uri = Some(RedirectUrl::new(redir_url)?);
        Ok(self)
    }

    pub fn get_redirect_uri(&self) -> Option<Url> {
        self.redirect_uri.clone().map(|x| x.url().clone())
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

    fn validate_redirect_uri(&self) -> crate::Result<RedirectUrl> {
        self.redirect_uri
            .clone()
            .ok_or(crate::Error::MissingRedirectUri)
    }

    /// https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow
    pub fn authorize_url(&mut self) -> crate::Result<(AuthrozationRequest, CsrfToken)> {
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

    /// https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow
    pub async fn exchange_code_for_token(
        &self,
        code: AuthorizationCode,
    ) -> crate::Result<TokenResponse<Token>> {
        let response = api_request(CodeTokenRequest::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            code,
            GrantType::AuthorizationCode,
            self.get_token_url(),
            self.validate_redirect_uri()?,
        ))
        .await?;

        Ok(TokenResponse::<Token>::new(
            response.status(),
            response.text().await?,
        ))
    }

    /// https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow
    #[cfg(feature = "oneshot-server")]
    pub async fn exchange_code(
        &mut self,
        code_state: crate::oneshot_server::CodeState,
        csrf_token: CsrfToken,
    ) -> crate::Result<TokenResponse<Token>> {
        match code_state.state {
            crate::oneshot_server::ServerStatus::Timeout => {
                Err(Error::TimeoutError("".to_string()))
            }
            crate::oneshot_server::ServerStatus::Shutdown => Err(Error::GraceFulShutdown),
            crate::oneshot_server::ServerStatus::Recive => {
                let received_csrf = code_state.csrf_token.ok_or(Error::ResponseCsrfTokenError)?;

                if received_csrf.secret() != csrf_token.secret() {
                    return Err(Error::CsrfTokenPartialEqError);
                }

                let code = code_state.code.ok_or(Error::MissingAuthorizationCode)?;
                self.exchange_code_for_token(code).await
            }
        }
    }
    /// https://dev.twitch.tv/docs/authentication/refresh-tokens/
    pub async fn exchange_refresh_token(
        &self,
        refresh_token: RefreshToken,
    ) -> crate::Result<TokenResponse<Token>> {
        let response = api_request(RefreshRequest::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::RefreshToken,
            refresh_token,
            self.get_token_url(),
        ))
        .await?;

        Ok(TokenResponse::<Token>::new(
            response.status(),
            response.text().await?,
        ))
    }

    /// https://dev.twitch.tv/docs/authentication/revoke-tokens/
    pub async fn revoke_token(&self, access_token: AccessToken) -> crate::Result<()> {
        api_request(RevokeRequest::new(
            access_token,
            self.client_id.clone(),
            self.get_revoke_url(),
        ))
        .await?;
        Ok(())
    }

    /// https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow
    pub async fn client_credentials(&self) -> crate::Result<TokenResponse<ClientCredentials>> {
        let response = api_request(ClientCredentialsRequest::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::ClientCredentials,
            self.get_token_url(),
        ))
        .await?;

        Ok(TokenResponse::<ClientCredentials>::new(
            response.status(),
            response.text().await?,
        ))
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

pub struct TokenResponse<T>
where
    T: DeserializeOwned,
{
    status_code: StatusCode,
    body: String,
    _phantom: PhantomData<T>,
}

impl<T> fmt::Debug for TokenResponse<T>
where
    T: DeserializeOwned,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenResponse")
            .field("status_code", &self.status_code)
            .field("body", &self.body)
            .finish()
    }
}

impl<T> TokenResponse<T>
where
    T: DeserializeOwned,
{
    pub fn new(status_code: StatusCode, body: String) -> Self {
        TokenResponse {
            status_code,
            body,
            _phantom: PhantomData,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status_code
    }

    pub fn raw_body(&self) -> &str {
        &self.body
    }

    pub fn parse_token(self) -> crate::Result<T> {
        match self.status_code {
            StatusCode::OK => serde_json::from_str(&self.body)
                .map_err(|e| Error::DeserializationError(e.to_string())),
            _ => {
                let error: TokenError = serde_json::from_str(&self.body)
                    .map_err(|e| Error::DeserializationError(e.to_string()))?;
                Err(Error::TokenError(error))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenError {
    #[serde(with = "http_serde::status_code")]
    status: StatusCode,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl TokenError {
    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn error_details(&self) -> Option<&str> {
        self.error.as_deref()
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token error ({}): {}{}",
            self.status,
            self.message,
            self.error
                .as_ref()
                .map(|e| format!(" - {}", e))
                .unwrap_or_default()
        )
    }
}

#[cfg(feature = "test")]
enum TestOauthType {
    User,
    App,
}
