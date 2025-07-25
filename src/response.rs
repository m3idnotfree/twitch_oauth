use std::{fmt, marker::PhantomData};

use reqwest::StatusCode;

use crate::{
    error::{self},
    types::{ClientCredentials, Token, ValidateToken},
    Error,
};

mod private {
    pub trait Sealed {}
}

pub trait ResponseState: private::Sealed {}

pub struct Unknown;
impl private::Sealed for Unknown {}
impl ResponseState for Unknown {}

pub struct ClientCredentialsResponse;
impl private::Sealed for ClientCredentialsResponse {}
impl ResponseState for ClientCredentialsResponse {}

pub struct TokenResponse;
impl private::Sealed for TokenResponse {}
impl ResponseState for TokenResponse {}

pub struct ValidateTokenResponse;
impl private::Sealed for ValidateTokenResponse {}
impl ResponseState for ValidateTokenResponse {}

pub struct NoContentResponse;
impl private::Sealed for NoContentResponse {}
impl ResponseState for NoContentResponse {}

pub struct Response<State = Unknown>
where
    State: ResponseState,
{
    inner: reqwest::Response,
    _state: PhantomData<State>,
}

impl<State> Response<State>
where
    State: ResponseState,
{
    pub(crate) fn new(response: reqwest::Response) -> Self {
        Self {
            inner: response,
            _state: PhantomData,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.inner.status()
    }

    pub fn is_success(&self) -> bool {
        self.inner.status().is_success()
    }

    pub fn headers(&self) -> &reqwest::header::HeaderMap {
        self.inner.headers()
    }

    pub async fn json<T>(self) -> Result<T, Error>
    where
        T: serde::de::DeserializeOwned,
    {
        self.inner.json::<T>().await.map_err(error::response::json)
    }

    pub async fn text(self) -> Result<String, Error> {
        self.inner.text().await.map_err(error::response::text)
    }

    pub fn into_inner(self) -> reqwest::Response {
        self.inner
    }
}

impl Response<ClientCredentialsResponse> {
    pub async fn client_credentials(self) -> Result<ClientCredentials, Error> {
        self.json::<ClientCredentials>().await
    }
}

impl Response<TokenResponse> {
    pub async fn token(self) -> Result<Token, Error> {
        self.json::<Token>().await
    }
}

impl Response<ValidateTokenResponse> {
    pub async fn validate_token(self) -> Result<ValidateToken, Error> {
        self.json::<ValidateToken>().await
    }
}

impl Response<NoContentResponse> {}

impl<State> fmt::Debug for Response<State>
where
    State: ResponseState,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status())
            .field("success", &self.is_success())
            .field("state_type", &std::any::type_name::<State>())
            .finish_non_exhaustive()
    }
}
