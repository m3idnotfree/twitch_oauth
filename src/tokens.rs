use asknothingx2_util::oauth::ClientId;
use chrono::Utc;
use serde::{Deserialize, Serialize, Serializer};

use crate::{scope::Scope, AccessToken, RefreshToken};

#[derive(Debug, Clone, Deserialize)]
pub struct UserToken {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: RefreshToken,
    #[serde(default, deserialize_with = "deserialize_scopes")]
    pub scope: Vec<Scope>,
    #[serde(default = "default_created_at")]
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

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateToken {
    pub client_id: ClientId,
    pub login: String,
    #[serde(default, deserialize_with = "deserialize_scopes")]
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

fn default_created_at() -> i64 {
    Utc::now().timestamp()
}

fn deserialize_scopes<'de, D>(deserializer: D) -> Result<Vec<Scope>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let scopes: Vec<String> = Vec::deserialize(deserializer)?;
    Ok(scopes
        .into_iter()
        .filter_map(|s| (!s.is_empty()).then(|| s.parse().ok()).flatten())
        .collect())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{UserToken, ValidateToken};

    #[test]
    fn user_token_deserialize_custom_scope() {
        let json = json!({
            "access_token":"d19bb4cb705d1f0",
            "refresh_token":"",
            "expires_in":86399,
            "scope":[""],
            "token_type":"bearer"
        });

        let token: UserToken = serde_json::from_value(json).unwrap();
        assert_eq!(token.scope.len(), 0);

        let json = json!({
            "access_token":"d19bb4cb705d1f0",
            "refresh_token":"",
            "expires_in":86399,
            "scope":[],
            "token_type":"bearer"
        });

        let token: UserToken = serde_json::from_value(json).unwrap();
        assert_eq!(token.scope.len(), 0);
    }

    #[test]
    fn validate_token_deserialize_custom_scope() {
        let json = json!({
          "client_id": "wbmytr93xzw8zbg0p1izqyzzc5mbiz",
          "login": "twitchdev",
          "scopes": [""],
          "user_id": "141981764",
          "expires_in": 5520838
        });

        let token: ValidateToken = serde_json::from_value(json).unwrap();
        assert_eq!(token.scopes.len(), 0);

        let json = json!({
          "client_id": "wbmytr93xzw8zbg0p1izqyzzc5mbiz",
          "login": "twitchdev",
          "scopes": [""],
          "user_id": "141981764",
          "expires_in": 5520838
        });

        let token: ValidateToken = serde_json::from_value(json).unwrap();
        assert_eq!(token.scopes.len(), 0);
    }
}
