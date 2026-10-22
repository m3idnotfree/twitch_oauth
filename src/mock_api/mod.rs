//! Testing with Mock API
//!
//! Defaults:
//! - port: 8080
//! - App access token URL: http://localhost:8080/auth/token
//! - User access token URL: http://localhost:8080/auth/authorize
//! - Validate URL: http://localhost:8080/auth/validate
//! - Units URL: http://localhost:8080/units
//!
//! ```no_run
//! use asknothingx2_http::client::Reqwest;
//! use twitch_oauth_token::{
//!     Client,
//!     mock_api::{MockApiService, MockUnits},
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let units = MockUnits::localhost(8080);
//!     let client = units.clients().await?.remove(0);
//!
//!     let oauth = Client::builder(client.id, client.secret)
//!         .http_client(MockApiService::localhost(
//!             Reqwest::new(reqwest::Client::new()),
//!             8080,
//!         ))
//!         .build();
//!
//!     // Get app token from mock API
//!     let app_token = oauth.app_access_token().await?;
//!
//!     let user = units.users().await?.remove(0);
//!     // Get user token from mock API
//!     let mut user_token_request = oauth.mock_user_token(&user.id);
//!     user_token_request.scopes_mut().update_user();
//!
//!     let user_token = user_token_request.send().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! <https://dev.twitch.tv/docs/cli/mock-api-command/>

mod units;
mod user_token;

pub use units::{MockClient, MockUnits, MockUser};
pub use user_token::MockUserTokenRequest;

use std::task::{Context, Poll};

use asknothingx2_http::Request;
use http::Uri;
use tower_service::Service;

/// Sends `https://id.twitch.tv/oauth2/{endpoint}` to `{base}/auth/{endpoint}`.
/// Other paths are kept.
#[derive(Debug, Clone)]
pub struct MockApiService<S> {
    inner: S,
    base: Uri,
}

impl<S> MockApiService<S> {
    pub fn localhost(inner: S, port: u16) -> Self {
        Self::new(inner, localhost(port))
    }

    pub fn new(inner: S, base: Uri) -> Self {
        Self { inner, base }
    }
}

fn localhost(port: u16) -> Uri {
    format!("http://localhost:{port}").parse().unwrap()
}

impl<S> Service<Request> for MockApiService<S>
where
    S: Service<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request) -> Self::Future {
        let mut parts = req.uri().clone().into_parts();
        let base = self.base.clone().into_parts();
        parts.scheme = base.scheme;
        parts.authority = base.authority;
        parts.path_and_query =
            parts
                .path_and_query
                .map(|pq| match pq.as_str().strip_prefix("/oauth2/") {
                    Some(rest) => format!("/auth/{rest}")
                        .parse()
                        .expect("only the path prefix is replaced"),
                    None => pq,
                });
        *req.uri_mut() = Uri::from_parts(parts).expect("the base has a scheme and a host");

        self.inner.call(req)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        convert::Infallible,
        future::{Ready, ready},
        task::{Context, Poll},
    };

    use asknothingx2_http::{Body, Request};
    use http::Uri;
    use tower_service::Service;

    use super::MockApiService;

    struct Echo;

    impl Service<Request> for Echo {
        type Response = Uri;
        type Error = Infallible;
        type Future = Ready<Result<Uri, Infallible>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: Request) -> Self::Future {
            ready(Ok(req.uri().clone()))
        }
    }

    fn call(mut svc: MockApiService<Echo>, uri: &str) -> String {
        let req = http::Request::post(uri).body(Body::empty()).unwrap();
        svc.call(req).into_inner().unwrap().to_string()
    }

    fn send(base: &str, uri: &str) -> String {
        call(MockApiService::new(Echo, base.parse().unwrap()), uri)
    }

    #[test]
    fn oauth2_path_to_auth() {
        assert_eq!(
            send(
                "http://localhost:8080",
                "https://id.twitch.tv/oauth2/authorize?client_id=id"
            ),
            "http://localhost:8080/auth/authorize?client_id=id"
        );
    }

    #[test]
    fn other_path_is_kept() {
        assert_eq!(
            send("http://127.0.0.1:9000/", "https://id.twitch.tv/units/users"),
            "http://127.0.0.1:9000/units/users"
        );
    }
}
