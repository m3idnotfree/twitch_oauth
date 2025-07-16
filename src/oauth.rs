use std::{fmt, marker::PhantomData, sync::LazyLock};

use asknothingx2_util::{
    api::{request::IntoRequestParts, Response},
    oauth::{
        AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, RefreshToken,
        RevocationUrl, TokenUrl, ValidateUrl,
    },
};
use http_serde::http::StatusCode;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    csrf, error, AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest, Error,
    RefreshRequest, RevokeRequest,
};

pub(crate) static AUTH_URL: LazyLock<AuthUrl> =
    LazyLock::new(|| AuthUrl::new("https://id.twitch.tv/oauth2/authorize".to_string()).unwrap());

pub(crate) static TOKEN_URL: LazyLock<TokenUrl> =
    LazyLock::new(|| TokenUrl::new("https://id.twitch.tv/oauth2/token".to_string()).unwrap());

pub(crate) static REVOKE_URL: LazyLock<RevocationUrl> =
    LazyLock::new(|| RevocationUrl::new("https://id.twitch.tv/oauth2/revoke".to_string()).unwrap());

pub(crate) static VALIDATE_URL: LazyLock<ValidateUrl> =
    LazyLock::new(|| ValidateUrl::new("https://id.twitch.tv/oauth2/validate".to_string()).unwrap());

mod private {
    pub trait Sealed {}
}

pub trait OauthState: private::Sealed {}

pub struct Unconfigured;
impl private::Sealed for Unconfigured {}
impl OauthState for Unconfigured {}

pub struct Configured;
impl private::Sealed for Configured {}
impl OauthState for Configured {}

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
///         "client_id",
///         "client_secret",
///     )?;
///     
///     let token = oauth.client_credentials().await?;
///     println!("Got token: {:?}", token);
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct TwitchOauth<HasRedirectUri = Unconfigured>
where
    HasRedirectUri: OauthState,
{
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: Option<RedirectUrl>,
    secret_key: [u8; 32],
    phanthom: PhantomData<HasRedirectUri>,
}

impl<State> TwitchOauth<State>
where
    State: OauthState,
{
    #[cfg(feature = "test")]
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    #[cfg(feature = "test")]
    pub fn client_secret(&self) -> &ClientSecret {
        &self.client_secret
    }

    pub fn get_redirect_uri(&self) -> Option<Url> {
        self.redirect_uri.clone().map(|uri| uri.url().clone())
    }

    pub fn get_auth_url(&self) -> &'static AuthUrl {
        &AUTH_URL
    }

    fn get_token_url(&self) -> &'static TokenUrl {
        &TOKEN_URL
    }

    fn get_revoke_url(&self) -> &'static RevocationUrl {
        &REVOKE_URL
    }

    fn validate_redirect_uri(&self) -> Result<RedirectUrl, Error> {
        self.redirect_uri
            .clone()
            .ok_or_else(error::oauth::missing_redirect_url)
    }

    /// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
    pub async fn exchange_refresh_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<Response, Error> {
        RefreshRequest::new(
            &self.client_id,
            &self.client_secret,
            refresh_token,
            self.get_token_url(),
        )
        .into_request_parts()
        .send()
        .await
        .map_err(error::network::request)
    }

    /// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
    pub async fn revoke_token(&self, access_token: &AccessToken) -> Result<Response, Error> {
        RevokeRequest::new(access_token, &self.client_id, self.get_revoke_url())
            .into_request_parts()
            .send()
            .await
            .map_err(error::network::request)
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
    pub async fn client_credentials(&self) -> Result<Response, Error> {
        ClientCredentialsRequest::new(&self.client_id.clone(), &self.client_secret.clone())
            .into_request_parts()
            .send()
            .await
            .map_err(error::network::request)
    }
}

impl TwitchOauth<Unconfigured> {
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Result<Self, Error> {
        Ok(Self {
            client_id: ClientId::new(client_id.into()),
            client_secret: ClientSecret::new(client_secret.into()),
            redirect_uri: None,
            secret_key: csrf::generate_secret_key(),
            phanthom: PhantomData,
        })
    }

    pub fn from_credentials(
        client_id: ClientId,
        client_secret: ClientSecret,
    ) -> Result<Self, Error> {
        Ok(Self {
            client_id,
            client_secret,
            redirect_uri: None,
            secret_key: csrf::generate_secret_key(),
            phanthom: PhantomData,
        })
    }

    pub fn set_redirect_uri(
        self,
        redir_url: impl Into<String>,
    ) -> Result<TwitchOauth<Configured>, Error> {
        Ok(TwitchOauth {
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri: Some(
                RedirectUrl::new(redir_url.into()).map_err(error::oauth::invalid_redirect_url)?,
            ),
            secret_key: self.secret_key,
            phanthom: PhantomData,
        })
    }
}
impl TwitchOauth<Configured> {
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub fn authorize_url<'a>(&'a self) -> Result<AuthrozationRequest<'a>, Error> {
        Ok(AuthrozationRequest::new(
            self.get_auth_url(),
            &self.client_id,
            self.redirect_uri.as_ref(),
            csrf::generate(&self.secret_key, Some(&self.client_id)),
        ))
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub async fn exchange_code_for_token(
        &self,
        code: AuthorizationCode,
    ) -> Result<Response, Error> {
        CodeTokenRequest::new(
            &self.client_id,
            &self.client_secret,
            &code,
            &self.validate_redirect_uri()?,
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
    ) -> Result<Response, Error> {
        match code_state.state {
            crate::oneshot_server::ServerStatus::Timeout => Err(error::server::timeout()),
            crate::oneshot_server::ServerStatus::Shutdown => Err(error::server::shutdown()),
            crate::oneshot_server::ServerStatus::Recive => {
                let received_csrf = code_state
                    .csrf_token
                    .ok_or_else(error::oauth::missing_csrf_token)?;

                if csrf::validate(&self.secret_key, &received_csrf, None, 6000) {
                    return Err(error::oauth::csrf_token_mismatch());
                }

                let code = code_state
                    .code
                    .ok_or_else(error::oauth::missing_auth_code)?;
                self.exchange_code_for_token(code).await
            }
        }
    }
}

#[cfg(feature = "test")]
impl<State> fmt::Debug for TwitchOauth<State>
where
    State: OauthState,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TwitchOauth")
            .field("client_id", &self.client_id)
            .field("client_secret", &self.client_secret)
            .field("redirect_uri", &self.redirect_uri)
            .finish()
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

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OAuth error {}: {}", self.status, self.message)?;
        if let Some(ref error) = self.error {
            write!(f, " ({error})")?;
        }
        Ok(())
    }
}

impl std::error::Error for TokenError {}
