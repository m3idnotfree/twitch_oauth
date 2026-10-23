use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Result as FmtResult},
};

use asknothingx2_http::{ClientService, HttpService, client::Reqwest};
use asknothingx2_util::oauth::ClientId;
use serde::Deserialize;
use tower_layer::{Identity, Layer, Stack};
use url::Url;

use crate::{
    DeviceCode, Error, Scope, UserToken,
    client::{json, read_json, send},
    error,
    request::{DeviceCodeRequest, DeviceTokenRequest},
    scope::{ScopesMut, scopes_mut},
    tokens::default_created_at,
};

pub struct DeviceAuthBuilder<L = Identity> {
    client_id: ClientId,
    scopes: HashSet<Scope>,
    layers: L,
}

impl DeviceAuthBuilder<Identity> {
    pub fn new(client_id: ClientId) -> Self {
        Self {
            client_id,
            scopes: HashSet::new(),
            layers: Identity::new(),
        }
    }
}

impl<L> DeviceAuthBuilder<L> {
    pub fn scopes<I>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = Scope>,
    {
        self.scopes.extend(scopes);
        self
    }

    pub fn layer<T>(self, layer: T) -> DeviceAuthBuilder<Stack<T, L>> {
        DeviceAuthBuilder {
            client_id: self.client_id,
            scopes: self.scopes,
            layers: Stack::new(layer, self.layers),
        }
    }
    pub fn build(self) -> DeviceAuth
    where
        L: Layer<Reqwest, Service: HttpService>,
    {
        self.http_client(crate::HTTP_CLIENT.clone())
    }

    pub fn http_client(self, http_client: reqwest::Client) -> DeviceAuth
    where
        L: Layer<Reqwest, Service: HttpService>,
    {
        self.service(Reqwest::new(http_client))
    }

    pub fn service<S>(self, service: S) -> DeviceAuth
    where
        L: Layer<S, Service: HttpService>,
    {
        DeviceAuth {
            client_id: self.client_id,
            scopes: self.scopes,
            svc: ClientService::new(self.layers.layer(service)),
        }
    }
}

pub struct DeviceAuth {
    client_id: ClientId,
    scopes: HashSet<Scope>,
    svc: ClientService,
}

impl DeviceAuth {
    pub fn new(client_id: ClientId) -> Self {
        DeviceAuthBuilder::new(client_id).build()
    }

    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes_mut(&mut self.scopes)
    }

    pub fn builder(client_id: ClientId) -> DeviceAuthBuilder {
        DeviceAuthBuilder::new(client_id)
    }

    /// Request a device code
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow>
    pub async fn device_code(&self) -> Result<DeviceCodeResponse, Error> {
        json(
            self.svc.clone(),
            DeviceCodeRequest::new(&self.client_id, &self.scopes_to_string()),
        )
        .await
    }

    /// Poll until the user authorizes or the device code expires
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow>
    pub async fn wait_for_token(&self, response: DeviceCodeResponse) -> Result<UserToken, Error> {
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
pub struct DeviceCodeResponse {
    pub device_code: DeviceCode,
    pub expires_in: u64,
    pub interval: u64,
    pub user_code: String,
    pub verification_uri: Url,
    #[serde(default = "default_created_at")]
    pub created_at: i64,
}

impl DeviceCodeResponse {
    pub fn verification_uri_without_code(&self) -> Url {
        let mut url = self.verification_uri.clone();
        url.set_query(None);
        url
    }
}

impl Display for DeviceCodeResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "DeviceCodeResponse(verification_uri: {}, user_code: {})",
            self.verification_uri, self.user_code
        )
    }
}
