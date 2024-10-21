mod authorize_request;
use asknothingx2_util::api::{CONTENT_TYPE_FORMENCODED, CONTENT_TYPE_JSON};
pub use authorize_request::*;
mod codetoken_request;
pub use codetoken_request::*;
mod refresh_request;
pub use refresh_request::*;
mod revoke_request;
pub use revoke_request::*;
mod validate_request;
pub use validate_request::*;
mod client_credentials;
pub use client_credentials::*;

use http::{
    header::{ACCEPT, CONTENT_TYPE},
    HeaderMap,
};

#[allow(non_snake_case)]
pub(crate) fn POST_formencoded_header() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.append(ACCEPT, CONTENT_TYPE_JSON());
    headers.append(CONTENT_TYPE, CONTENT_TYPE_FORMENCODED());
    headers
}
