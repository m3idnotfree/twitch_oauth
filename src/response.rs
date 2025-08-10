use std::{fmt, marker::PhantomData};

use asknothingx2_util::api::HeaderMap;
use reqwest::StatusCode;

use crate::{
    error::{self},
    tokens::{AppToken, UserToken, ValidateToken},
    Error,
};

mod private {
    pub trait Sealed {}
}

pub trait ResponseType: private::Sealed {}

pub struct Unknown;
impl private::Sealed for Unknown {}
impl ResponseType for Unknown {}

pub struct AppTokenResponse;
impl private::Sealed for AppTokenResponse {}
impl ResponseType for AppTokenResponse {}

pub struct UserTokenResponse;
impl private::Sealed for UserTokenResponse {}
impl ResponseType for UserTokenResponse {}

pub struct ValidateTokenResponse;
impl private::Sealed for ValidateTokenResponse {}
impl ResponseType for ValidateTokenResponse {}

pub struct NoContentResponse;
impl private::Sealed for NoContentResponse {}
impl ResponseType for NoContentResponse {}

pub struct Response<State = Unknown>
where
    State: ResponseType,
{
    inner: reqwest::Response,
    _state: PhantomData<State>,
}

impl<State> Response<State>
where
    State: ResponseType,
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

    pub fn headers(&self) -> &HeaderMap {
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

impl Response<AppTokenResponse> {
    pub async fn app_token(self) -> Result<AppToken, Error> {
        self.json::<AppToken>().await
    }
}

impl Response<UserTokenResponse> {
    pub async fn user_token(self) -> Result<UserToken, Error> {
        self.json::<UserToken>().await
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
    State: ResponseType,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status())
            .field("success", &self.is_success())
            .field("state_type", &std::any::type_name::<State>())
            .finish_non_exhaustive()
    }
}
