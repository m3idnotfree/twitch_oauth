use std::{
    collections::HashSet,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};

use asknothingx2_util::oauth::ClientId;
use reqwest::Client;
use serde::Deserialize;
use url::Url;

use crate::{
    DeviceCode, DeviceUrl, Error, Scope, TokenUrl, UserToken, error,
    oauth::{TOKEN_URL, execute, read_json},
    request::{CLIENT_ID, GRANT_TYPE},
    scope::{ScopesMut, scopes_mut},
    tokens::default_created_at,
    types::GrantType,
};

const DEVICE_URL: &str = "https://id.twitch.tv/oauth2/device";

pub struct DeviceAuth {
    client_id: ClientId,
    scopes: HashSet<Scope>,
    client: Client,
    device_url: DeviceUrl,
    token_url: TokenUrl,
}

impl DeviceAuth {
    pub fn new(client_id: impl Into<ClientId>) -> Self {
        Self {
            client_id: client_id.into(),
            scopes: HashSet::new(),
            client: crate::client::get().clone(),
            device_url: DeviceUrl::from_str(DEVICE_URL).unwrap(),
            token_url: TokenUrl::from_str(TOKEN_URL).unwrap(),
        }
    }

    pub fn with_scopes(mut self, scopes: HashSet<Scope>) -> Self {
        self.scopes = scopes;
        self
    }

    pub fn with_client(mut self, client: Client) -> Self {
        self.client = client;
        self
    }

    pub fn with_device_url(mut self, device_url: DeviceUrl) -> Self {
        self.device_url = device_url;
        self
    }

    pub fn with_token_url(mut self, token_url: TokenUrl) -> Self {
        self.token_url = token_url;
        self
    }

    pub fn scopes_mut(&mut self) -> ScopesMut<'_> {
        scopes_mut(&mut self.scopes)
    }

    /// Request a device code
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow>
    pub async fn request(&self) -> Result<DeviceAuthResponse, Error> {
        let form = reqwest::multipart::Form::new()
            .text(CLIENT_ID, self.client_id.to_string())
            .text("scopes", self.scopes_to_string());

        let req = self.client.post(self.device_url.to_url()).multipart(form);
        let resp = execute(error::Operation::DeviceRequest, req).await?;

        read_json(error::Operation::DeviceRequest, resp).await
    }

    /// Poll for the user token
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow>
    pub async fn poll(&self, response: DeviceAuthResponse) -> Result<UserToken, Error> {
        use chrono::Utc;
        use std::time::Duration;
        use tokio::time::sleep;

        let deadline = response.created_at + response.expires_in as i64;

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
                return Err(error::device_code_expired(error::Operation::DevicePoll));
            }

            #[cfg(feature = "tracing")]
            {
                poll_count += 1;
            }

            let form = reqwest::multipart::Form::new()
                .text(CLIENT_ID, self.client_id.to_string())
                .text("scopes", self.scopes_to_string())
                .text("device_code", response.device_code.secret().to_string())
                .text(GRANT_TYPE, GrantType::DeviceCode.as_str());

            let req = self.client.post(self.token_url.to_url()).multipart(form);

            match execute(error::Operation::DevicePoll, req).await {
                Ok(resp) => {
                    let token = read_json(error::Operation::DevicePoll, resp).await?;

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
