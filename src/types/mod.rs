mod scope;

pub use scope::{
    AdsScopes, AnalyticsScopes, BitsScopes, CCLsScopes, ChannelPointsScopes, ChannelScopes,
    CharityScopes, ChatScopes, ClipsScopes, ConduitsScopes, EntitlementScopes, EventSubScopes,
    ExtensionsScopes, GamesScopes, GoalsScopes, GuestStarScopes, HypeTrainScopes, IRCScopes,
    ModerationScopes, PollsScopes, PredictionsScopes, RaidsScopes, ScheduleScopes, Scope,
    ScopesMut, SearchScopes, StreamsScopes, SubscriptionsScopes, TagsScopes, TeamsScopes,
    UsersScopes, VideosScopes, WhispersScopes,
};

use std::{collections::HashSet, fmt};

use asknothingx2_util::oauth::AuthorizationCode;
use serde::{Deserialize, Serialize};

use crate::{AccessToken, RefreshToken};

pub fn scopes_mut(scopes: &mut HashSet<Scope>) -> ScopesMut<'_> {
    scope::new(scopes)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: RefreshToken,
    #[serde(default)]
    pub scope: Vec<Scope>,
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
