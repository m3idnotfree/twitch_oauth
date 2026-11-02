use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    marker::PhantomData,
};

use asknothingx2_http::{ClientService, HttpService, client::Reqwest};
use tower_layer::{Identity, Layer, Stack};

use crate::{
    AccessToken, AuthorizationRequest, ClientId, ClientSecret, Error, RedirectUri, RefreshToken,
    VerifiedCode, error,
    request::{
        ClientCredentialsRequest, ExchangeCodeRequest, IntoRequest, RefreshRequest, RevokeRequest,
        ValidateRequest,
    },
};

pub struct ClientBuilder<L = Identity> {
    client_id: ClientId,
    client_secret: ClientSecret,
    layers: L,
}

impl ClientBuilder<Identity> {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Self {
        Self {
            client_id,
            client_secret,
            layers: Identity::new(),
        }
    }
}

impl<L> ClientBuilder<L> {
    pub fn layer<T>(self, layer: T) -> ClientBuilder<Stack<T, L>> {
        ClientBuilder {
            client_id: self.client_id,
            client_secret: self.client_secret,
            layers: Stack::new(layer, self.layers),
        }
    }

    pub fn build(self) -> Client
    where
        L: Layer<Reqwest, Service: HttpService>,
    {
        self.http_client(crate::HTTP_CLIENT.clone())
    }

    pub fn http_client(self, http_client: reqwest::Client) -> Client
    where
        L: Layer<Reqwest, Service: HttpService>,
    {
        self.service(Reqwest::new(http_client))
    }

    pub fn service<S>(self, service: S) -> Client
    where
        L: Layer<S, Service: HttpService>,
    {
        Client {
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri: (),
            svc: ClientService::new(self.layers.layer(service)),
            phanthom: PhantomData,
        }
    }
}

mod private {
    pub trait Sealed {}
}

pub trait OauthFlow: private::Sealed + Debug + Clone + Copy {
    type RedirectUri: Debug;
}

#[derive(Debug, Clone, Copy)]
pub struct AppAuth;
impl private::Sealed for AppAuth {}
impl OauthFlow for AppAuth {
    type RedirectUri = ();
}

#[derive(Debug, Clone, Copy)]
pub struct UserAuth;
impl private::Sealed for UserAuth {}
impl OauthFlow for UserAuth {
    type RedirectUri = RedirectUri;
}

/// OAuth client for Twitch
///
/// - [`Client<AppAuth>`]: Client credentials grant flow
/// - [`Client<UserAuth>`]: Authorization code grant flow, created with [`Client::with_redirect_uri`]
///
/// See the [crate documentation](crate)
#[derive(Clone)]
pub struct Client<Flow = AppAuth>
where
    Flow: OauthFlow,
{
    client_id: ClientId,
    client_secret: ClientSecret,
    redirect_uri: Flow::RedirectUri,
    pub(crate) svc: ClientService,
    phanthom: PhantomData<Flow>,
}

impl<Flow> Client<Flow>
where
    Flow: OauthFlow,
{
    pub fn client_id(&self) -> &ClientId {
        &self.client_id
    }

    #[deprecated(note = "accessing client_secret is discouraged for security reasons")]
    pub fn client_secret(&self) -> &ClientSecret {
        &self.client_secret
    }

    /// Update the client secret at runtime
    pub fn set_client_secret(&mut self, client_secret: ClientSecret) {
        self.client_secret = client_secret;
    }

    /// <https://dev.twitch.tv/docs/authentication/refresh-tokens/>
    pub async fn refresh_access_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<crate::UserToken, Error> {
        json(
            self.svc.clone(),
            RefreshRequest::new(&self.client_id, &self.client_secret, refresh_token),
        )
        .await
    }

    /// <https://dev.twitch.tv/docs/authentication/revoke-tokens/>
    pub async fn revoke_access_token(&self, access_token: &AccessToken) -> Result<(), Error> {
        let _resp = send(
            self.svc.clone(),
            RevokeRequest::new(access_token, &self.client_id),
        )
        .await?;

        Ok(())
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
    pub async fn app_access_token(&self) -> Result<crate::AppToken, Error> {
        json(
            self.svc.clone(),
            ClientCredentialsRequest::new(&self.client_id, &self.client_secret),
        )
        .await
    }

    /// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
    pub async fn validate_access_token(
        &self,
        access_token: &AccessToken,
    ) -> Result<crate::TokenInfo, Error> {
        json(self.svc.clone(), ValidateRequest::new(access_token)).await
    }
}

impl Client<AppAuth> {
    pub fn new(client_id: ClientId, client_secret: ClientSecret) -> Self {
        ClientBuilder::new(client_id, client_secret).build()
    }

    pub fn with_redirect_uri(self, redirect_uri: RedirectUri) -> Client<UserAuth> {
        Client {
            client_id: self.client_id,
            client_secret: self.client_secret,
            redirect_uri,
            svc: self.svc,
            phanthom: PhantomData,
        }
    }

    pub fn builder(client_id: ClientId, client_secret: ClientSecret) -> ClientBuilder {
        ClientBuilder::new(client_id, client_secret)
    }
}

impl Client<UserAuth> {
    pub fn redirect_uri(&self) -> &RedirectUri {
        &self.redirect_uri
    }

    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub fn authorization_url<'a>(&'a self) -> AuthorizationRequest<'a> {
        AuthorizationRequest::new(&self.client_id, &self.redirect_uri)
    }

    /// Get the [`VerifiedCode`] from [`AuthorizationResponse::verify`](crate::AuthorizationResponse::verify).
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#authorization-code-grant-flow>
    pub async fn exchange_code(&self, code: &VerifiedCode) -> Result<crate::UserToken, Error> {
        json(
            self.svc.clone(),
            ExchangeCodeRequest::new(
                &self.client_id,
                &self.client_secret,
                code.code(),
                &self.redirect_uri,
            ),
        )
        .await
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
