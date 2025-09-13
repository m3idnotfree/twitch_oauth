use asknothingx2_util::oauth::{AccessToken, RefreshToken};
use chrono::Utc;
use serde::{Deserialize, Serialize, Serializer};

use crate::scope::Scope;

#[derive(Debug, Clone)]
pub struct UserToken {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: RefreshToken,
    pub scope: Vec<Scope>,
    pub created_at: i64,
}

impl UserToken {
    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now >= (self.created_at + self.expires_in as i64)
    }
}

impl Serialize for UserToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Token", 5)?;
        state.serialize_field("access_token", &self.access_token)?;
        state.serialize_field("expires_in", &self.expires_in)?;
        state.serialize_field("token_type", &self.token_type)?;
        state.serialize_field("refresh_token", &self.refresh_token)?;
        state.serialize_field("scope", &self.scope)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for UserToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: AccessToken,
            expires_in: u64,
            token_type: String,
            refresh_token: RefreshToken,
            #[serde(default)]
            scope: Vec<Scope>,
            #[serde(default)]
            created_at: Option<i64>,
        }

        let resp = TokenResponse::deserialize(deserializer)?;
        Ok(UserToken {
            access_token: resp.access_token,
            expires_in: resp.expires_in,
            token_type: resp.token_type,
            refresh_token: resp.refresh_token,
            scope: resp.scope,
            created_at: resp.created_at.unwrap_or_else(|| Utc::now().timestamp()),
        })
    }
}

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateToken {
    pub client_id: String,
    pub login: String,
    pub scopes: Vec<Scope>,
    pub user_id: String,
    pub expires_in: u64,
}

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppToken {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
}
