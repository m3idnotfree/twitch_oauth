mod scope;

use chrono::Utc;
pub use scope::{
    AdsScopes, AnalyticsScopes, BitsScopes, CCLScopes, ChannelPointsScopes, ChannelScopes,
    CharityScopes, ChatScopes, ClipsScopes, ConduitsScopes, EntitlementScopes, EventSubScopes,
    ExtensionsScopes, GamesScopes, GoalsScopes, GuestStarScopes, HypeTrainScopes, IRCScopes,
    ModerationScopes, PollsScopes, PredictionsScopes, RaidsScopes, ScheduleScopes, Scope,
    ScopesMut, SearchScopes, StreamsScopes, SubscriptionsScopes, TagsScopes, TeamsScopes,
    UsersScopes, VideosScopes, WhispersScopes,
};

use std::{collections::HashSet, fmt};

use asknothingx2_util::oauth::{AccessToken, AuthorizationCode, RefreshToken};
use serde::{Deserialize, Serialize, Serializer};

pub fn scopes_mut(scopes: &mut HashSet<Scope>) -> ScopesMut<'_> {
    scope::new(scopes)
}

#[derive(Debug)]
pub struct Token {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: RefreshToken,
    pub scope: Vec<Scope>,
    pub created_at: i64,
}

impl Token {
    pub fn is_scope_empty(&self) -> bool {
        if self.scope.is_empty() {
            return true;
        }
        if self.scope.len() > 1 {
            return false;
        }

        *self.scope.first().unwrap() == Scope::EmptyString
    }

    pub fn is_expired(&self) -> bool {
        let now = Utc::now().timestamp();
        now >= (self.created_at + self.expires_in as i64)
    }
}

impl Serialize for Token {
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

impl<'de> Deserialize<'de> for Token {
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
        }

        let resp = TokenResponse::deserialize(deserializer)?;
        Ok(Token {
            access_token: resp.access_token,
            expires_in: resp.expires_in,
            token_type: resp.token_type,
            refresh_token: resp.refresh_token,
            scope: resp.scope,
            created_at: Utc::now().timestamp(),
        })
    }
}

/// <https://dev.twitch.tv/docs/authentication/validate-tokens/>
#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateToken {
    pub client_id: String,
    pub login: String,
    pub scopes: Vec<Scope>,
    pub user_id: String,
    pub expires_in: u64,
}

/// <https://dev.twitch.tv/docs/authentication/getting-tokens-oauth/#client-credentials-grant-flow>
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientCredentials {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
}

#[derive(Debug, Clone, Copy)]
pub enum ResponseType {
    Token,
    Code,
}

impl ResponseType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Token => "token",
            Self::Code => "code",
        }
    }
}

#[derive(Debug)]
pub enum GrantType {
    ClientCredentials,
    AuthorizationCode,
    RefreshToken,
    #[cfg(feature = "test")]
    UserToken,
}

impl fmt::Display for GrantType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClientCredentials => write!(f, "client_credentials"),
            Self::AuthorizationCode => write!(f, "authorization_code"),
            Self::RefreshToken => write!(f, "refresh_token"),
            #[cfg(feature = "test")]
            Self::UserToken => write!(f, "user_token"),
        }
    }
}

impl GrantType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientCredentials => "client_credentials",
            Self::AuthorizationCode => "authorization_code",
            Self::RefreshToken => "refresh_token",
            #[cfg(feature = "test")]
            Self::UserToken => "user_token",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: AuthorizationCode,
    pub scope: String,
    pub state: String,
}
