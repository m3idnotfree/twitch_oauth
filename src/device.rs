use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Result as FmtResult},
};

use asknothingx2_http::{ClientService, Request, client::Reqwest};
use asknothingx2_util::oauth::ClientId;
use bytes::Bytes;
use serde::Deserialize;
use tower_service::Service;
use url::Url;

use crate::{
    DeviceCode, Error, Scope, UserToken,
    client::{json, read_json, send},
    error::{self, BoxError},
    request::{DeviceCodeRequest, DeviceTokenRequest},
    scope::{ScopesMut, scopes_mut},
    tokens::default_created_at,
};

pub struct DeviceAuth {
    client_id: ClientId,
    scopes: HashSet<Scope>,
    svc: ClientService,
}

impl DeviceAuth {
    pub fn new(client_id: ClientId) -> Self {
        Self {
            client_id,
            scopes: HashSet::new(),
            svc: ClientService::new(Reqwest::new(crate::default_client())),
        }
    }

    pub fn with_http_client<S, B>(mut self, service: S) -> Self
    where
        S: Service<Request, Response = http::Response<B>> + Clone + Send + Sync + 'static,
        S::Error: Into<BoxError>,
        S::Future: Send + 'static,
        B: http_body::Body<Data = Bytes> + Send + Sync + 'static,
        B::Error: Into<BoxError>,
    {
        self.svc = ClientService::new(service);
        self
    }

    pub fn with_scopes(mut self, scopes: HashSet<Scope>) -> Self {
        self.scopes = scopes;
        self
    }

    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes_mut(&mut self.scopes)
    }

    /// Request a device code
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow>
    pub async fn request(&self) -> Result<DeviceAuthResponse, Error> {
        json(
            self.svc.clone(),
            DeviceCodeRequest::new(&self.client_id, &self.scopes_to_string()),
        )
        .await
    }

    /// Poll for the user token
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow>
    pub async fn poll(&self, response: DeviceAuthResponse) -> Result<UserToken, Error> {
        use chrono::Utc;
        use std::time::Duration;
        use tokio::time::sleep;

        let operation = error::Operation::DeviceToken;
        let deadline = response.created_at + response.expires_in as i64;
        let scopes = self.scopes_to_string();

        #[cfg(feature = "tracing")]
        tracing::debug!(
            client_id = %self.client_id,
            interval_secs = response.interval,
            remaining_secs = deadline - Utc::now().timestamp(),
            "starting device code poll"
        );

        #[cfg(feature = "tracing")]
        let mut poll_count: u32 = 0;
        loop {
            sleep(Duration::from_secs(response.interval)).await;

            if Utc::now().timestamp() >= deadline {
                #[cfg(feature = "tracing")]
                tracing::debug!(
                    client_id = %self.client_id,
                    poll_count,
                    "device code expired"
                );
                return Err(error::device_code_expired(operation));
            }

            #[cfg(feature = "tracing")]
            {
                poll_count += 1;
            }

            let req = DeviceTokenRequest::new(&self.client_id, &scopes, &response.device_code);

            match send(self.svc.clone(), req).await {
                Ok(resp) => {
                    let token = read_json(operation, resp).await?;

                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        client_id = %self.client_id,
                        poll_count,
                        "device code token obtained"
                    );

                    return Ok(token);
                }
                Err(e) if e.message() == Some("authorization_pending") => {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(
                        client_id = %self.client_id,
                        poll_count,
                        "authorization pending"
                    );
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn scopes_to_string(&self) -> String {
        self.scopes
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceAuthResponse {
    pub device_code: DeviceCode,
    pub expires_in: u64,
    pub interval: u64,
    pub user_code: String,
    pub verification_uri: Url,
    #[serde(default = "default_created_at")]
    pub created_at: i64,
}

impl DeviceAuthResponse {
    pub fn verification_uri_without_code(&self) -> Url {
        let mut url = self.verification_uri.clone();
        url.set_query(None);
        url
    }
}

impl Display for DeviceAuthResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "DeviceAuthResponse(verification_uri: {}, user_code: {})",
            self.verification_uri, self.user_code
        )
    }
}
