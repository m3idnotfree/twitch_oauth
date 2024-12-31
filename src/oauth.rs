use std::{fmt, marker::PhantomData};

use asknothingx2_util::{
    api::api_request,
    oauth::{
        AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
        RefreshToken, RevocationUrl, TokenUrl, ValidateUrl,
    },
};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use url::Url;

use crate::{
    types::{ClientCredentials, GrantType, ResponseType, Token, ValidateToken},
    AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest, Error, RefreshRequest,
    RevokeRequest, ValidateRequest,
};

const BASE_URL: &str = "https://id.twitch.tv/oauth2";
const AUTH: &str = "authorize";
const TOKEN: &str = "token";
const REVOKE: &str = "revoke";
const VALIDATE: &str = "validate";

#[derive(Debug)]
pub struct TwitchOauth {
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: Option<RedirectUrl>,
    #[cfg(feature = "test")]
    test_url: crate::test_url::TestUrlHold,
}

impl TwitchOauth {
    pub fn new<T: Into<String>>(
        client_id: T,
        client_secret: T,
        redirect_uri: Option<T>,
    ) -> crate::Result<Self> {
        Ok(Self {
            client_id: ClientId::new(client_id.into()),
            client_secret: ClientSecret::new(client_secret.into()),
            //redirect_url: RedirectUrl::new(format!("http://localhost:{}", PORT)).unwrap(),
            redirect_uri: match redirect_uri {
                Some(uri) => Some(RedirectUrl::new(uri.into())?),
                None => None,
            },
            #[cfg(feature = "test")]
            test_url: crate::test_url::TestUrlHold::default(),
        })
    }

    pub fn set_redirect_uri<T: Into<String>>(mut self, redir_url: T) -> crate::Result<Self> {
        self.redirect_uri = Some(RedirectUrl::new(redir_url.into())?);
        Ok(self)
    }

    pub fn get_redirect_uri(&self) -> Option<Url> {
        self.redirect_uri.clone().map(|x| x.url().clone())
    }

    pub fn get_auth_url(&self) -> AuthUrl {
        #[cfg(feature = "test")]
        if let Some(url) = &self.test_url.get_test_url() {
            return AuthUrl::new(url.to_string()).unwrap();
        }

        AuthUrl::new(format!("{BASE_URL}/{AUTH}")).unwrap()
    }

    fn get_token_url(&self) -> TokenUrl {
        TokenUrl::new(format!("{BASE_URL}/{TOKEN}")).unwrap()
    }

    fn get_revoke_url(&self) -> RevocationUrl {
        RevocationUrl::new(format!("{BASE_URL}/{REVOKE}")).unwrap()
    }

    fn get_validate_url(&self) -> ValidateUrl {
        ValidateUrl::new(format!("{BASE_URL}/{VALIDATE}")).unwrap()
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
    /// https://dev.twitch.tv/docs/authentication/validate-tokens/
    pub async fn validate_token(
        &self,
        access_token: AccessToken,
    ) -> crate::Result<TokenResponse<ValidateToken>> {
        let response = api_request(ValidateRequest::new(
            access_token.clone(),
            self.get_validate_url(),
        ))
        .await?;

        Ok(TokenResponse::<ValidateToken>::new(
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
    pub fn with_url<T: Into<String>>(&mut self, url: T) -> &mut Self {
        self.test_url.with_url(url.into());
        self
    }

    /// Getting a user access token
    #[cfg(feature = "test")]
    pub fn user_token<T: Into<String>>(&self, user_id: T) -> crate::test_url::TestAccessToken {
        crate::test_url::TestAccessToken::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::UserToken,
            user_id.into(),
            std::collections::HashSet::new(),
            self.get_auth_url(),
        )
    }
    /// Getting an app access token
    #[cfg(feature = "test")]
    pub fn app_token(&self) -> crate::test_url::TestAccessToken {
        crate::test_url::TestAccessToken::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::ClientCredentials,
            "",
            std::collections::HashSet::new(),
            self.get_auth_url(),
        )
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
