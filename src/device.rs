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
    error,
    oauth::TOKEN_URL,
    request::{CLIENT_ID, GRANT_TYPE},
    scope::{scopes_mut, ScopesMut},
    tokens::default_created_at,
    types::GrantType,
    DeviceCode, DeviceUrl, Error, Scope, TokenUrl, UserToken,
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

        let resp = self
            .client
            .post(self.device_url.to_url())
            .multipart(form)
            .send()
            .await
            .map_err(error::network::request)?;

        if resp.status().is_success() {
            resp.json::<DeviceAuthResponse>().await.map_err(Error::from)
        } else {
            let status = resp.status().as_u16();
            let v = resp.bytes().await?;
            let body = String::from_utf8_lossy(&v).to_string();
            Err(error::oauth::http_error(status, body))
        }
    }

    /// Poll for the user token
    ///
    /// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#device-code-grant-flow>
    pub async fn poll(&self, response: DeviceAuthResponse) -> Result<UserToken, Error> {
        use chrono::Utc;
        use std::time::Duration;
        use tokio::time::sleep;
        use tracing::{debug, trace, warn};

        let deadline = response.created_at + response.expires_in as i64;

        debug!(
            client_id = %self.client_id,
            expires_in = response.expires_in,
            interval = response.interval,
            created_at = %format_timestamp(response.created_at),
            deadline = %format_timestamp(deadline),
            "starting device code poll"
        );

        let mut poll_count: u32 = 0;
        loop {
            sleep(Duration::from_secs(response.interval)).await;

            if Utc::now().timestamp() >= deadline {
                debug!(
                    client_id = %self.client_id,
                    poll_count,
                    "device code expired"
                );
                return Err(error::device_code::timeout());
            }

            poll_count += 1;

            let form = reqwest::multipart::Form::new()
                .text(CLIENT_ID, self.client_id.to_string())
                .text("scopes", self.scopes_to_string())
                .text("device_code", response.device_code.secret().to_string())
                .text(GRANT_TYPE, GrantType::DeviceCode.as_str());

            let resp = self
                .client
                .post(self.token_url.to_url())
                .multipart(form)
                .send()
                .await?;

            if resp.status().is_success() {
                debug!(
                    client_id = %self.client_id,
                    poll_count,
                    "device code token obtained"
                );
                return Ok(resp.json::<UserToken>().await?);
            }

            let err = resp.json::<DeviceErrorResponse>().await?;
            if err.is_pending() {
                trace!(
                    client_id = %self.client_id,
                    poll_count,
                    "authorization pending"
                );
                continue;
            }

            warn!(
                client_id = %self.client_id,
                status = err.status,
                message = %err.message,
                poll_count,
                "device code flow error"
            );
            return Err(err.into_error());
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

#[derive(Deserialize)]
struct DeviceErrorResponse {
    status: u16,
    message: String,
}

impl DeviceErrorResponse {
    pub fn into_error(self) -> Error {
        error::device_code::flow_error(self.status, self.message)
    }

    pub fn is_pending(&self) -> bool {
        self.message == "authorization_pending"
    }
}

fn format_timestamp(timestamp: i64) -> String {
    use chrono::TimeZone;
    chrono::Local
        .timestamp_opt(timestamp, 0)
        .single()
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| timestamp.to_string())
}
