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

pub const CLIENT_ID: &str = "client_id";
pub const GRANT_TYPE: &str = "grant_type";

const CLIENT_SECRET: &str = "client_secret";

pub(crate) trait IntoRequestBuilder {
    const OPERATION: crate::error::Operation;
    type Error: Into<crate::error::BoxError>;

    fn into_request_builder(
        self,
        client: &reqwest::Client,
    ) -> Result<reqwest::RequestBuilder, Self::Error>;
}
