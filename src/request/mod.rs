mod authorize_request;
mod client_credentials;
mod codetoken_request;
mod refresh_request;
mod revoke_request;
mod validate_request;

pub use authorize_request::AuthrozationRequest;
pub use client_credentials::ClientCredentialsRequest;
pub use codetoken_request::CodeTokenRequest;
pub use refresh_request::RefreshRequest;
pub use revoke_request::RevokeRequest;
pub use validate_request::{validate_token, ValidateRequest};

use reqwest::{Client, RequestBuilder};

const CLIENT_ID: &str = "client_id";
const CLIENT_SECRET: &str = "client_secret";
const GRANT_TYPE: &str = "grant_type";

pub trait IntoRequestBuilder {
    type Error;
    fn into_request_builder(self, client: &Client) -> Result<RequestBuilder, Self::Error>;
}
