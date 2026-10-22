use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};

use asknothingx2_http::{ClientService, Request, client::Reqwest};
use bytes::Bytes;
use tower_service::Service;

use crate::{
    AccessToken, AuthorizationCode, AuthrozationRequest, ClientId, ClientSecret, Error,
    RedirectUrl, RefreshToken,
    csrf::{self, CsrfConfig},
    device::DeviceAuth,
    error::{self, BoxError},
    request::{
        ClientCredentialsRequest, ExchangeCodeRequest, IntoRequest, RefreshRequest, RevokeRequest,
        ValidateRequest,
    },
};

pub struct ClientBuilder {
    client_id: ClientId,
    client_secret: ClientSecret,
    csrf_config: Option<CsrfConfig>,
    secret_key: Option<[u8; 32]>,
    svc: Option<ClientService>,
}

impl ClientBuilder {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Self {
        Self {
            client_id,
            client_secret,
            svc: None,
            csrf_config: None,
            secret_key: None,
        }
    }

    /// Configure CSRF token validation settings
    ///
    /// This controls how CSRF tokens are validated during the OAuth flow.
    /// Tokens use HMAC-SHA256 signatures with timestamp validation.
    ///
    /// Defaults:
    /// - max_age: 1800s (30 minutes)
    /// - clock_skew: None (no tolerance for time differences)
    pub fn csrf_config(mut self, config: CsrfConfig) -> Self {
        self.csrf_config = Some(config);
        self
    }

    /// Set custom secret key for CSRF token generation
    ///
    /// By default, a random secret key is generated automatically for each `Client` instance.
    /// Set custom secret key for CSRF token generation and validation
    pub fn secret_key(mut self, secret_key: [u8; 32]) -> Self {
        self.secret_key = Some(secret_key);
        self
    }

    pub fn http_client<S, B>(mut self, service: S) -> Self
    where
        S: Service<Request, Response = http::Response<B>> + Clone + Send + Sync + 'static,
        S::Error: Into<BoxError>,
        S::Future: Send + 'static,
        B: http_body::Body<Data = Bytes> + Send + Sync + 'static,
        B::Error: Into<BoxError>,
    {
        self.svc = Some(ClientService::new(service));
        self
    }

    pub fn build(self) -> Client {
        Client {
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri: (),
            secret_key: self.secret_key.unwrap_or_else(csrf::generate_secret_key),
            csrf_config: self.csrf_config.unwrap_or_default(),
            svc: self
                .svc
                .unwrap_or_else(|| ClientService::new(Reqwest::new(crate::default_client()))),
            phanthom: PhantomData,
        }
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
/// use twitch_oauth_token::Client;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let oauth = Client::new("client_id", "client_secret");
///     
///     let token = oauth.app_access_token().await?;
///     
///     println!("App token: {}", token.access_token.secret());
///     Ok(())
/// }
/// ```
///
/// **User authentication** (for user login flows):
/// ```no_run
/// use std::str::FromStr;
/// use twitch_oauth_token::{Client, RedirectUrl};
///
/// #[tokio::main]
/// async fn main() -> Result<(), twitch_oauth_token::Error> {
///     let oauth = Client::new("your_client_id", "your_client_secret")
///         .with_redirect_uri(RedirectUrl::from_str("http://localhost:3000/auth/callback").unwrap());
///     
///     // Step 1: Get authorization URL (send user here)
///     let auth_request = oauth.authorization_url();
///     println!("Visit: {}", auth_request.url());
///     
///     // Step 2: After user authorizes, exchange code for token
///     // let token = oauth.exchange_code(code, state).await?;
///     
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct Client<Flow = AppAuth>
where
    Flow: OauthFlow,
{
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: Flow::RedirectUrl,
    secret_key: [u8; 32],
    csrf_config: CsrfConfig,
    pub(crate) svc: ClientService,
    phanthom: PhantomData<Flow>,
}

impl<Flow> Client<Flow>
where
    Flow: OauthFlow,
{
    /// Get the client ID
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    /// Get the client secret
    #[deprecated(note = "accessing client_secret is discouraged for security reasons")]
    pub fn client_secret(&self) -> &ClientSecret {
        &self.client_secret
    }

    /// Update the client secret at runtime
    ///
    /// Use this when you need to rotate credentials in a running application,
    /// especially in `Arc<RwLock<Client>>` patterns.
    ///
    /// For initial configuration during construction, use [`Client::new`] instead.
    pub fn set_client_secret(&mut self, client_secret: ClientSecret) {
        self.client_secret = client_secret;
    }

    /// Update CSRF token validation settings at runtime
    ///
    /// Use this to adjust CSRF validation behavior in a running application,
    /// especially in `Arc<RwLock<Client>>` patterns.
    ///
    /// For initial configuration during construction, use [`ClientBuilder::csrf_config`] instead.
    pub fn set_csrf_config(&mut self, config: CsrfConfig) {
        self.csrf_config = config;
    }

    /// **Refresh an access token** using a refresh token
    ///
    /// # Example
    /// ```no_run
    /// # use twitch_oauth_token::{Client, RefreshToken};
    /// # async fn run(oauth: Client, refresh_token: RefreshToken) -> Result<(), twitch_oauth_token::Error> {
    /// let new_token = oauth.refresh_access_token(refresh_token).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
    pub async fn refresh_access_token(
        &self,
        refresh_token: RefreshToken,
    ) -> Result<crate::UserToken, Error> {
        json(
            self.svc.clone(),
            RefreshRequest::new(&self.client_id, &self.client_secret, refresh_token),
        )
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
    /// # use twitch_oauth_token::{Client, AccessToken};
    /// # async fn run(oauth: Client, access_token: AccessToken) -> Result<(), twitch_oauth_token::Error> {
    /// oauth.revoke_access_token(&access_token).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
    pub async fn revoke_access_token(&self, access_token: &AccessToken) -> Result<(), Error> {
        let _resp = send(
            self.svc.clone(),
            RevokeRequest::new(access_token, &self.client_id),
        )
        .await?;

        Ok(())
    }

    /// **Get an app access token** (Client Credentials Flow)
    ///
    /// App tokens are used for server-to-server API calls that don't
    /// require a specific user context. They're simpler than user tokens
    /// but can only access public data.
    ///
    /// # Example
    /// ```no_run
    /// # use twitch_oauth_token::Client;
    /// # async fn run() -> Result<(), twitch_oauth_token::Error> {
    /// let oauth = Client::new("client_id", "client_secret");
    ///
    /// let token = oauth.app_access_token().await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
    pub async fn app_access_token(&self) -> Result<crate::AppToken, Error> {
        json(
            self.svc.clone(),
            ClientCredentialsRequest::new(&self.client_id, &self.client_secret),
        )
        .await
    }

    /// **Validate access token**
    ///
    /// # Example
    /// ```no_run
    /// # use twitch_oauth_token::{Client, AccessToken};
    /// # async fn run(oauth: Client, access_token: AccessToken) -> Result<(), twitch_oauth_token::Error> {
    /// let user_info = oauth.validate_access_token(&access_token).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
    pub async fn validate_access_token(
        &self,
        access_token: &AccessToken,
    ) -> Result<crate::TokenInfo, Error> {
        json(self.svc.clone(), ValidateRequest::new(access_token)).await
    }
}

impl Client<AppAuth> {
    /// Create OAuth client for app authentication
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        ClientBuilder::new(
            ClientId::from(client_id.into()),
            ClientSecret::from(client_secret.into()),
        )
        .build()
    }

    /// Upgrade to user authentication by adding redirect URI
    pub fn with_redirect_uri(self, redirect_uri: RedirectUrl) -> Client<UserAuth> {
        Client {
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri,
            secret_key: self.secret_key,
            csrf_config: self.csrf_config,
            svc: self.svc,
            phanthom: PhantomData,
        }
    }

    /// Create OAuth client from existing credentials (advanced usage)
    ///
    /// Most users should use [`Client::new()`] instead.
    pub fn from_credentials(client_id: ClientId, client_secret: ClientSecret) -> Self {
        ClientBuilder::new(client_id, client_secret).build()
    }

    /// Create Device code grant flow
    pub fn device_auth(client_id: ClientId) -> DeviceAuth {
        DeviceAuth::new(client_id)
    }

    pub fn builder(client_id: ClientId, client_secret: ClientSecret) -> ClientBuilder {
        ClientBuilder::new(client_id, client_secret)
    }
}

impl Client<UserAuth> {
    pub fn get_redirect_uri(&self) -> &RedirectUrl {
        &self.redirect_uri
    }

    /// **Generate authorization URL** for user login (Step 1 of user auth)
    ///
    /// This creates a URL that you send users to for Twitch login.
    /// The URL includes:
    /// - Your client ID and redirect URI
    /// - Requested scopes (permissions)
    /// - CSRF protection via HMAC-SHA256 signed state parameter with timestamp
    ///
    /// # Example
    /// ```no_run
    /// # use std::str::FromStr;
    /// # use twitch_oauth_token::{Client, RedirectUrl};
    /// # async fn run() -> Result<(), twitch_oauth_token::Error> {
    /// let oauth = Client::new("client_id", "client_secret")
    ///     .with_redirect_uri(RedirectUrl::from_str("http://localhost:3000/auth/callback").unwrap());
    ///
    /// let mut auth_request = oauth.authorization_url();
    /// auth_request.scopes_mut().chat_api();
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
    ///     AuthCallback,
    ///     Client,
    ///     UserAuth
    /// };
    ///
    /// async fn handle_callback(
    ///     oauth: &Client<UserAuth>,
    ///     oauth_callback: AuthCallback,
    /// ) -> Result<(), twitch_oauth_token::Error> {
    ///     let token = oauth
    ///         .exchange_code(oauth_callback.code, oauth_callback.state)
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub async fn exchange_code(
        &self,
        code: AuthorizationCode,
        state: String,
    ) -> Result<crate::UserToken, Error> {
        csrf::verify_with_config(
            &self.secret_key,
            &state,
            Some(&self.client_id),
            &self.csrf_config,
        )
        .map_err(|e| error::csrf(error::Operation::ExchangeCode, e))?;

        json(
            self.svc.clone(),
            ExchangeCodeRequest::new(
                &self.client_id,
                &self.client_secret,
                code,
                &self.redirect_uri,
            ),
        )
        .await
    }

    /// Update the CSRF secret key at runtime
    ///
    /// Use this to rotate CSRF secret keys in a running application,
    /// especially in `Arc<RwLock<Client>>` patterns or multi-server deployments.
    ///
    /// For initial configuration during construction, use [`ClientBuilder::secret_key`] instead.
    pub fn set_secret_key(&mut self, secret_key: [u8; 32]) {
        self.secret_key = secret_key;
    }
}

impl Display for Client<AppAuth> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Client(client_id: {})", self.client_id)
    }
}

impl Display for Client<UserAuth> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Client(client_id: {}, redirect_uri: {})",
            self.client_id, self.redirect_uri
        )
    }
}

impl<Flow> Debug for Client<Flow>
where
    Flow: OauthFlow,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Client")
            .field("client_id", &self.client_id)
            .field("client_secret", &self.client_secret)
            .field("redirect_uri", &self.redirect_uri)
            .finish()
    }
}

pub(crate) async fn json<T, R>(svc: ClientService, request: T) -> Result<R, Error>
where
    T: IntoRequest,
    R: serde::de::DeserializeOwned,
{
    let resp = send(svc, request).await?;
    read_json(T::OPERATION, resp).await
}

pub(crate) async fn send<T>(
    svc: ClientService,
    request: T,
) -> Result<asknothingx2_http::Response, Error>
where
    T: IntoRequest,
{
    let req = request
        .into_request()
        .map_err(|e| error::build(T::OPERATION, e))?;
    execute(T::OPERATION, svc, req).await
}

pub(crate) async fn execute(
    operation: error::Operation,
    svc: ClientService,
    req: asknothingx2_http::Request,
) -> Result<asknothingx2_http::Response, Error> {
    let resp = svc
        .execute(req)
        .await
        .map_err(|e| error::request(operation, e.into_inner()))?;

    if !resp.status().is_success() {
        let status = resp.status().as_u16();
        let body = resp.into_body().collect_bytes().await.unwrap_or_default();

        return Err(error::api(operation, status, &body));
    }

    Ok(resp)
}

pub(crate) async fn read_json<T>(
    operation: error::Operation,
    resp: asknothingx2_http::Response,
) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let body = resp
        .into_body()
        .collect_bytes()
        .await
        .map_err(|e| error::body(operation, e.into_inner()))?;
    decode(operation, &body)
}

pub(crate) fn decode<T>(operation: error::Operation, body: &[u8]) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let mut de = serde_json::Deserializer::from_slice(body);

    let value = match serde_path_to_error::deserialize(&mut de) {
        Ok(value) => value,
        Err(e) => return Err(error::parse(operation, e)),
    };

    match de.end() {
        Ok(()) => Ok(value),
        Err(e) => Err(error::parse(operation, e)),
    }
}
