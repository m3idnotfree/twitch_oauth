use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
    str::FromStr,
    sync::OnceLock,
};

use asknothingx2_util::api::IntoRequestBuilder;
use reqwest::Client;

use crate::{
    csrf::{self, CsrfConfig},
    error,
    request::{
        ClientCredentialsRequest, CodeTokenRequest, RefreshRequest, RevokeRequest, ValidateRequest,
    },
    response::ResponseType,
    types::GrantType,
    AccessToken, AppTokenResponse, AuthUrl, AuthorizationCode, AuthrozationRequest, ClientId,
    ClientSecret, Error, NoContentResponse, RedirectUrl, RefreshToken, Response, RevocationUrl,
    TokenUrl, UserTokenResponse, ValidateTokenResponse, ValidateUrl,
};

pub const AUTH_URL: &str = "https://id.twitch.tv/oauth2/authorize";
pub const TOKEN_URL: &str = "https://id.twitch.tv/oauth2/token";
pub const REVOKE_URL: &str = "https://id.twitch.tv/oauth2/revoke";
pub const VALIDATE_URL: &str = "https://id.twitch.tv/oauth2/validate";

static CLIENT: OnceLock<Client> = OnceLock::new();

/// **Global HTTP Client Management for Oauth Request**
///
/// This module manages a single, shared HTTP client used by all TwitchOauth instances
/// in your application. This design provides several benefits:
///
/// - **Connection pooling**: Reuses HTTP connections across OAuth operations
/// - **Consistent configuration**: Same timeouts, headers, etc. for all requests
/// - **Memory efficiency**: One client instead of many
/// - **Thread safety**: Can be safely used from multiple threads
///
/// # Default Behavior (No Setup Required)
///
/// If you don't call `setup()`, a default client is created automatically with:
/// - User-Agent: "twitch-oauth/1.0"
/// - Request timeout: 60s, Connect timeout: 10s
/// - Connections: 30 max per host, 90s idle timeout
/// - TLS: 1.2+ minimum, strict validation
/// - HTTPS: Enforced (HTTP blocked)
/// - HTTP/2: Required (no HTTP/1.1 fallback)
/// - Redirects: Up to 5 allowed
/// - Cookies: Not saved, Referer: Not sent (strict security)
/// - Headers: Accept JSON, no-cache control
///
/// This works fine for most applications.
///
/// # Basic Usage (No Setup)
/// ```no_run
/// # use twitch_oauth_token::TwitchOauth;
/// # async fn run() -> Result<(), twitch_oauth_token::Error> {
/// let oauth = TwitchOauth::new("client_id", "client_secret");
/// let token = oauth.app_access_token().await?;
/// # Ok(())
/// # }
/// ```
///
/// # Custom Configuration
/// ```no_run
/// # use twitch_oauth_token::client;
/// # use twitch_oauth_token::TwitchOauth;
/// # use std::time::Duration;
///
/// # async fn run() -> Result<(), twitch_oauth_token::Error> {
/// // Configure once at startup
/// client::setup(|preset| {
///     Ok(preset
///         .timeouts(Duration::from_secs(60), Duration::from_secs(30))
///         .connections(10, Duration::from_secs(90))
///         .default_headers(|headers| {
///             headers.accept_json().content_type_json();
///             Ok(())
///         })?
///         .user_agent("user-agent/1.0"))
/// })?;
///
/// // Now all OAuth instances use your custom client
/// let oauth = TwitchOauth::new("client_id", "client_secret");
/// # Ok(())
/// # }
/// ```
pub mod client {
    use asknothingx2_util::api::preset::{self, Preset};
    use reqwest::Client;

    use crate::{error, Error};

    use super::CLIENT;

    /// Configure the global HTTP client used for all OAuth requests
    ///
    /// This should be called once at application startup if you need custom
    /// timeouts, proxies, or other HTTP client configuration.
    ///
    /// # Example
    /// ```no_run
    /// # use twitch_oauth_token::client;
    /// # use std::time::Duration;
    /// # fn run() -> Result<(), twitch_oauth_token::Error> {
    ///  client::setup(|preset| {
    ///      Ok(preset
    ///          .timeouts(Duration::from_secs(60), Duration::from_secs(30))
    ///          .connections(10, Duration::from_secs(90))
    ///          .default_headers(|headers| {
    ///              headers.accept_json().content_type_json();
    ///              Ok(())
    ///          })?
    ///          .user_agent("user-agent/1.0"))
    ///  })?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn setup<F>(f: F) -> Result<(), Error>
    where
        F: FnOnce(Preset) -> Result<Preset, asknothingx2_util::api::Error>,
    {
        if CLIENT.get().is_some() {
            return Err(error::client_setup::already_initialized());
        }
        let preset = preset::authentication("twitch-oauth/1.0");

        let preset = f(preset).map_err(|e| {
            error::client_setup::from_preset_error("preset configuration failed", e)
        })?;

        let client = preset
            .build_client()
            .map_err(error::client_setup::build_failed)?;

        CLIENT
            .set(client)
            .map_err(|_| error::client_setup::already_initialized())?;

        Ok(())
    }

    /// Get the global HTTP client (creates default if not configured)
    pub fn get() -> &'static Client {
        CLIENT.get_or_init(|| {
            preset::authentication("twitch-oauth/1.0")
                .build_client()
                .expect("failed to build default http client")
        })
    }
}

mod private {
    pub trait Sealed {}
}

/// Marker trait for OAuth flow types - prevents external implementations
pub trait OauthFlow: private::Sealed + Debug + Clone + Copy {
    type RedirectUrl: Debug;
}

/// **App Authentication** (Client Credentials Flow)
///
/// Use this flow when your application needs to:
/// - Make API calls on behalf of your app (not users)
/// - Access public data (streams, games, users)
/// - Run as a backend service without user interaction
///
/// **Cannot do:**
/// - Access user-specific data (follows, subscriptions)
/// - Perform actions on behalf of users
/// - Handle user login flows
///
#[derive(Debug, Clone, Copy)]
pub struct AppAuth;
impl private::Sealed for AppAuth {}
impl OauthFlow for AppAuth {
    type RedirectUrl = ();
}

/// **User Authentication** (Authorization Code Flow)
///
/// Use this flow when your application needs to:
/// - Allow users to log in with their Twitch account
/// - Access user-specific data (follows, subscriptions, chat)
/// - Perform actions on behalf of users
/// - Get long-lived refresh tokens
///
/// **Requires:**
/// - A redirect URI (where Twitch sends the user after login)
/// - User interaction (they must visit the auth URL)
///
#[derive(Debug, Clone, Copy)]
pub struct UserAuth;
impl private::Sealed for UserAuth {}
impl OauthFlow for UserAuth {
    type RedirectUrl = RedirectUrl;
}

/// **OAuth client for Twitch API authentication**
///
/// The client supports two authentication flows:
/// - **AppAuth**: For server-to-server communication (no user interaction)
/// - **UserAuth**: For user authentication flows (requires redirect URI)
///
/// **App authentication** (most common for backend services):
/// ```no_run
/// use twitch_oauth_token::TwitchOauth;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let oauth = TwitchOauth::new("client_id", "client_secret");
///     
///     let response = oauth.app_access_token().await?;
///     let token = response.app_token().await?;
///     
///     println!("App token: {}", token.access_token.secret());
///     Ok(())
/// }
/// ```
///
/// **User authentication** (for user login flows):
/// ```no_run
/// use std::str::FromStr;
/// use twitch_oauth_token::{TwitchOauth, RedirectUrl};
///
/// #[tokio::main]
/// async fn main() -> Result<(), twitch_oauth_token::Error> {
///     let oauth = TwitchOauth::new("your_client_id", "your_client_secret")
///         .set_redirect_uri(RedirectUrl::from_str("http://localhost:3000/auth/callback").unwrap());
///     
///     // Step 1: Get authorization URL (send user here)
///     let auth_request = oauth.authorization_url();
///     println!("Visit: {}", auth_request.url());
///     
///     // Step 2: After user authorizes, exchange code for token
///     // let token = oauth.user_access_token(code, state).await?;
///     
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct TwitchOauth<Flow = AppAuth>
where
    Flow: OauthFlow,
{
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: Flow::RedirectUrl,
    secret_key: [u8; 32],
    client: Client,
    token_url: TokenUrl,
    auth_url: AuthUrl,
    revoke_url: RevocationUrl,
    validate_url: ValidateUrl,
    csrf_config: CsrfConfig,
    phanthom: PhantomData<Flow>,
}

impl<Flow> TwitchOauth<Flow>
where
    Flow: OauthFlow,
{
    /// Override the HTTP client
    ///
    /// Note: This only affects this OAuth instance, not the global client.
    /// For global configuration, use [`client::setup()`] instead.
    pub fn set_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    /// Configure CSRF token validation settings
    ///
    /// This controls how CSRF tokens are validated during the OAuth flow.
    ///
    /// Defaults:
    /// - max_age: 1800s (30 minutes)
    /// - clock_skew: None (no tolerance for time differences)
    pub fn set_csrf_config(mut self, config: CsrfConfig) -> Self {
        self.csrf_config = config;
        self
    }

    pub async fn send<T, R>(&self, request: T) -> Result<Response<R>, T::Error>
    where
        T: IntoRequestBuilder<Error = Error>,
        R: ResponseType,
    {
        let resp = request
            .into_request_builder(&self.client)?
            .send()
            .await
            .map_err(error::network::request)?;

        if !resp.status().is_success() {
            let status = resp.status();
            match resp.text().await {
                Ok(body) => {
                    return Err(error::oauth::server_error(format!("HTTP {status}: {body}")));
                }
                Err(e) => {
                    return Err(error::oauth::server_error(format!(
                        "HTTP {status} - Failed to read error response: {e}"
                    )));
                }
            }
        }

        Ok(Response::new(resp))
    }

    /// **Refresh an access token** using a refresh token
    ///
    /// # Example
    /// ```no_run
    /// # use twitch_oauth_token::{TwitchOauth, RefreshToken};
    /// # async fn run(oauth: TwitchOauth, refresh_token: RefreshToken) -> Result<(), twitch_oauth_token::Error> {
    /// let response = oauth.refresh_access_token(refresh_token).await?;
    /// let new_token = response.user_token().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
    pub async fn refresh_access_token(
        &self,
        refresh_token: RefreshToken,
    ) -> Result<Response<UserTokenResponse>, Error> {
        self.send(RefreshRequest::new(
            &self.client_id,
            &self.client_secret,
            refresh_token,
            &self.token_url,
        ))
        .await
    }

    /// **Revoke/invalidate an access token**
    ///
    /// This immediately invalidates a token, preventing further use.
    /// Use this when:
    /// - User logs out of your application
    /// - You detect a security issue
    /// - You're shutting down/cleaning up
    ///
    /// # Example
    /// ```no_run
    /// # use twitch_oauth_token::{TwitchOauth, AccessToken};
    /// # async fn run(oauth: TwitchOauth, access_token: AccessToken) -> Result<(), twitch_oauth_token::Error> {
    /// oauth.revoke_access_token(&access_token).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
    pub async fn revoke_access_token(
        &self,
        access_token: &AccessToken,
    ) -> Result<Response<NoContentResponse>, Error> {
        self.send(RevokeRequest::new(
            access_token,
            &self.client_id,
            &self.revoke_url,
        ))
        .await
    }

    /// **Get an app access token** (Client Credentials Flow)
    ///
    /// App tokens are used for server-to-server API calls that don't
    /// require a specific user context. They're simpler than user tokens
    /// but can only access public data.
    ///
    /// # Example
    /// ```no_run
    /// # use twitch_oauth_token::TwitchOauth;
    /// # async fn run() -> Result<(), twitch_oauth_token::Error> {
    /// let oauth = TwitchOauth::new("client_id", "client_secret");
    ///
    /// let response = oauth.app_access_token().await?;
    /// let credentials = response.app_token().await?;
    ///
    /// let token = credentials.access_token;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
    pub async fn app_access_token(&self) -> Result<Response<AppTokenResponse>, Error> {
        self.send(ClientCredentialsRequest::new(
            &self.client_id,
            &self.client_secret,
            GrantType::ClientCredentials,
            &self.token_url,
        ))
        .await
    }

    /// **Validate access token**
    ///
    /// # Example
    /// ```no_run
    /// # use twitch_oauth_token::{TwitchOauth, AccessToken};
    /// # async fn run(oauth: TwitchOauth, access_token: AccessToken) -> Result<(), twitch_oauth_token::Error> {
    /// let response = oauth.validate_access_token(&access_token).await?;
    /// let validation = response.validate_token().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
    pub async fn validate_access_token(
        &self,
        access_token: &AccessToken,
    ) -> Result<Response<ValidateTokenResponse>, Error> {
        self.send(ValidateRequest::new(access_token, &self.validate_url))
            .await
    }
}

impl TwitchOauth<AppAuth> {
    /// Create OAuth client for app authentication
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            client_id: ClientId::from(client_id.into()),
            client_secret: ClientSecret::from(client_secret.into()),
            redirect_uri: (),
            secret_key: csrf::generate_secret_key(),
            token_url: TokenUrl::from_str(TOKEN_URL).unwrap(),
            auth_url: AuthUrl::from_str(AUTH_URL).unwrap(),
            revoke_url: RevocationUrl::from_str(REVOKE_URL).unwrap(),
            validate_url: ValidateUrl::from_str(VALIDATE_URL).unwrap(),
            client: client::get().clone(),
            csrf_config: CsrfConfig::default(),
            phanthom: PhantomData,
        }
    }

    /// Upgrade to user authentication by adding redirect URI
    pub fn set_redirect_uri(self, redirect_uri: RedirectUrl) -> TwitchOauth<UserAuth> {
        TwitchOauth {
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri,
            secret_key: self.secret_key,
            token_url: self.token_url,
            auth_url: self.auth_url,
            revoke_url: self.revoke_url,
            validate_url: self.validate_url,
            client: self.client,
            csrf_config: self.csrf_config,
            phanthom: PhantomData,
        }
    }

    /// Create OAuth client from existing credentials (advanced usage)
    ///
    /// Most users should use `TwitchOauth::new()` instead.
    pub fn from_credentials(client_id: ClientId, client_secret: ClientSecret) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri: (),
            secret_key: csrf::generate_secret_key(),
            client: client::get().clone(),
            token_url: TokenUrl::from_str(TOKEN_URL).unwrap(),
            auth_url: AuthUrl::from_str(AUTH_URL).unwrap(),
            revoke_url: RevocationUrl::from_str(REVOKE_URL).unwrap(),
            validate_url: ValidateUrl::from_str(VALIDATE_URL).unwrap(),
            csrf_config: CsrfConfig::default(),
            phanthom: PhantomData,
        }
    }
}

impl TwitchOauth<UserAuth> {
    pub fn get_redirect_uri(&self) -> &RedirectUrl {
        &self.redirect_uri
    }

    /// **Generate authorization URL** for user login (Step 1 of user auth)
    ///
    /// This creates a URL that you send users to for Twitch login.
    /// The URL includes:
    /// - Your client ID and redirect URI
    /// - Requested scopes (permissions)
    /// - CSRF protection (state parameter)
    ///
    /// # Example
    /// ```no_run
    /// # use std::str::FromStr;
    /// # use twitch_oauth_token::{scope::ChatScopes, TwitchOauth, RedirectUrl};
    /// # async fn run() -> Result<(), twitch_oauth_token::Error> {
    /// let oauth = TwitchOauth::new("client_id", "client_secret")
    ///     .set_redirect_uri(RedirectUrl::from_str("http://localhost:3000/auth/callback").unwrap());
    ///
    /// let mut auth_request = oauth.authorization_url();
    /// auth_request.scopes_mut().chat_api_as_user();
    ///
    /// let auth_url = auth_request.url();
    /// println!("{}", auth_url);
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub fn authorization_url<'a>(&'a self) -> AuthrozationRequest<'a> {
        AuthrozationRequest::new(
            &self.auth_url,
            &self.client_id,
            &self.redirect_uri,
            csrf::generate(&self.secret_key, Some(&self.client_id)),
        )
    }

    /// **Exchange authorization code for user access token** (Step 2 of user auth)
    ///
    /// After the user authorizes your app, Twitch redirects them back to your
    /// redirect URI with a `code`, `state` and `scope` parameter. Use this method to
    /// exchange that code for actual access tokens.
    ///
    /// # Example Callback Handler
    /// ```no_run
    /// use twitch_oauth_token::{
    ///     AuthorizationCode,
    ///     OAuthCallbackQuery,
    ///     TwitchOauth,
    ///     UserAuth
    /// };
    ///
    /// async fn handle_callback(
    ///     oauth: &TwitchOauth<UserAuth>,
    ///     oauth_callback: OAuthCallbackQuery,
    /// ) -> Result<(), twitch_oauth_token::Error> {
    ///     let response = oauth
    ///         .user_access_token(oauth_callback.code, oauth_callback.state)
    ///         .await?;
    ///     let token = response.user_token().await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub async fn user_access_token(
        &self,
        code: AuthorizationCode,
        state: String,
    ) -> Result<Response<UserTokenResponse>, Error> {
        if !csrf::validate_with_config(
            &self.secret_key,
            &state,
            Some(&self.client_id),
            &self.csrf_config,
        ) {
            return Err(error::oauth::csrf_token_mismatch());
        }
        self.send(CodeTokenRequest::new(
            &self.client_id,
            &self.client_secret,
            code,
            &self.redirect_uri,
            &self.token_url,
        ))
        .await
    }

    /// Set custom secret key for CSRF token generation
    ///
    /// By default, a random secret key is generated automatically.
    pub fn set_secret_key(mut self, secret_key: [u8; 32]) -> Self {
        self.secret_key = secret_key;
        self
    }
}

#[cfg(feature = "test")]
impl<Flow> TwitchOauth<Flow>
where
    Flow: OauthFlow,
{
    pub(crate) fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    pub(crate) fn client_secret(&self) -> &ClientSecret {
        &self.client_secret
    }

    pub fn set_auth_url(mut self, auth_url: AuthUrl) -> Self {
        self.auth_url = auth_url;
        self
    }

    pub fn set_token_url(mut self, token_url: TokenUrl) -> Self {
        self.token_url = token_url;
        self
    }

    pub fn set_revoke_url(mut self, revoke_url: RevocationUrl) -> Self {
        self.revoke_url = revoke_url;
        self
    }

    pub fn set_validate_url(mut self, validate_url: ValidateUrl) -> Self {
        self.validate_url = validate_url;
        self
    }

    pub fn with_test(self) -> crate::test_oauth::TwitchOauthTest<Flow> {
        crate::test_oauth::TwitchOauthTest::new(self)
    }
}

impl Display for TwitchOauth<AppAuth> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "TwitchOauth(client_id: {})", self.client_id)
    }
}

impl Display for TwitchOauth<UserAuth> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "TwitchOauth(client_id: {}, redirect_uri: {})",
            self.client_id, self.redirect_uri
        )
    }
}

impl<Flow> Debug for TwitchOauth<Flow>
where
    Flow: OauthFlow,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("TwitchOauth")
            .field("client_id", &self.client_id)
            .field("client_secret", &self.client_secret)
            .field("redirect_uri", &self.redirect_uri)
            .field("token_url", &self.token_url)
            .field("auth_url", &self.auth_url)
            .field("revoke_url", &self.revoke_url)
            .finish()
    }
}
