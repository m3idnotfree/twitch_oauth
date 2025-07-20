use std::{fmt, marker::PhantomData, sync::LazyLock};

use asknothingx2_util::{
    api::preset,
    oauth::{
        AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, RedirectUrl, RefreshToken,
        RevocationUrl, TokenUrl, ValidateUrl,
    },
};
use http_serde::http::StatusCode;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    csrf, error, request::IntoRequestBuilder, AuthrozationRequest, ClientCredentialsRequest,
    CodeTokenRequest, Error, RefreshRequest, RevokeRequest,
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
/// ```no_run
/// use twitch_oauth_token::TwitchOauth;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
///     let oauth = TwitchOauth::new(
///         "client_id",
///         "client_secret",
///     );
///     
///     let token = oauth.client_credentials().json().await?;
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
    client: Client,
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

    fn validate_redirect_uri(&self) -> Result<&RedirectUrl, Error> {
        self.redirect_uri
            .as_ref()
            .ok_or_else(error::oauth::missing_redirect_url)
    }

    /// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
    pub fn exchange_refresh_token<'a>(&'a self, refresh_token: RefreshToken) -> RefreshRequest<'a> {
        RefreshRequest::new(
            &self.client_id,
            &self.client_secret,
            refresh_token,
            self.get_token_url(),
            &self.client,
        )
    }

    /// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
    pub async fn revoke_token(&self, access_token: &AccessToken) -> Result<Response, Error> {
        RevokeRequest::new(access_token, &self.client_id, self.get_revoke_url())
            .into_request_builder(&self.client)?
            .send()
            .await
            .map_err(error::network::request)
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
    pub fn client_credentials(&self) -> ClientCredentialsRequest {
        ClientCredentialsRequest::new(&self.client_id, &self.client_secret, &self.client)
    }
}

impl TwitchOauth<Unconfigured> {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: ClientId::new(client_id.into()),
            client_secret: ClientSecret::new(client_secret.into()),
            redirect_uri: None,
            secret_key: csrf::generate_secret_key(),
            phanthom: PhantomData,
            client: preset::for_auth("twitch-oauth/1.0").build_client().unwrap(),
        }
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
            client: preset::for_auth("twitch-oauth/1.0").build_client().unwrap(),
        })
    }

    pub fn set_client(mut self, client: Client) -> Self {
        self.client = client;
        self
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
            client: self.client,
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
            self.validate_redirect_uri()?,
            csrf::generate(&self.secret_key, Some(&self.client_id)),
        ))
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub fn exchange_code<'a>(
        &'a self,
        code: AuthorizationCode,
        state: String,
    ) -> Result<CodeTokenRequest<'a>, Error> {
        if csrf::validate(&self.secret_key, &state, None, 600) {
            return Err(error::oauth::csrf_token_mismatch());
        }

        Ok(CodeTokenRequest::new(
            &self.client_id,
            &self.client_secret,
            code,
            self.validate_redirect_uri()?,
            &self.client,
        ))
    }
}

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
