use std::ops::Deref;

use asknothingx2_http::Body;
use asknothingx2_util::api::mime_type::Application;
use http::header::{ACCEPT, CONTENT_TYPE};

use crate::{
    ClientId, DeviceCode,
    request::{CLIENT_ID, DEVICE_URL, GRANT_TYPE, IntoRequest, TOKEN_URL},
    types::GrantType,
};

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow>
#[derive(Debug)]
pub struct DeviceCodeRequest<'a> {
    client_id: &'a ClientId,
    scopes: &'a str,
}

impl<'a> DeviceCodeRequest<'a> {
    pub fn new(client_id: &'a ClientId, scopes: &'a str) -> Self {
        Self { client_id, scopes }
    }
}

impl IntoRequest for DeviceCodeRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::DeviceCode;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        Ok(http::Request::post(DEVICE_URL)
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(Body::from(super::form(&[
                (CLIENT_ID, self.client_id.deref()),
                ("scopes", self.scopes),
            ])?))?)
    }
}

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow>
#[derive(Debug)]
pub struct DeviceTokenRequest<'a> {
    client_id: &'a ClientId,
    scopes: &'a str,
    device_code: &'a DeviceCode,
}

impl<'a> DeviceTokenRequest<'a> {
    pub fn new(client_id: &'a ClientId, scopes: &'a str, device_code: &'a DeviceCode) -> Self {
        Self {
            client_id,
            scopes,
            device_code,
        }
    }
}

impl IntoRequest for DeviceTokenRequest<'_> {
    const OPERATION: crate::error::Operation = crate::error::Operation::DeviceToken;

    fn into_request(self) -> Result<asknothingx2_http::Request, crate::error::BoxError> {
        Ok(http::Request::post(TOKEN_URL)
            .header(ACCEPT, Application::Json)
            .header(CONTENT_TYPE, Application::FormUrlEncoded)
            .body(Body::from(super::form(&[
                (CLIENT_ID, self.client_id.deref()),
                ("scopes", self.scopes),
                ("device_code", self.device_code.secret()),
                (GRANT_TYPE, GrantType::DeviceCode.as_str()),
            ])?))?)
    }
}
