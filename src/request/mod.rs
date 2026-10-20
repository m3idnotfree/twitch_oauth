mod authorize_request;
mod client_credentials;
mod exchange_code;
mod refresh_request;
mod revoke_request;
mod validate_request;

pub use authorize_request::AuthrozationRequest;
pub use client_credentials::ClientCredentialsRequest;
pub use exchange_code::ExchangeCodeRequest;
pub use refresh_request::RefreshRequest;
pub use revoke_request::RevokeRequest;
pub use validate_request::{ValidateRequest, validate_access_token};

use asknothingx2_http::Request;

use crate::error::BoxError;

pub const CLIENT_ID: &str = "client_id";
pub const GRANT_TYPE: &str = "grant_type";

const CLIENT_SECRET: &str = "client_secret";

pub(crate) trait IntoRequest {
    const OPERATION: crate::error::Operation;

    fn into_request(self) -> Result<Request, BoxError>;
}

pub(super) fn form(form: &[(&str, &str)]) -> Result<String, serde_urlencoded::ser::Error> {
    serde_urlencoded::to_string(form)
}
