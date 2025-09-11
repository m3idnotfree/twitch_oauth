use asknothingx2_util::oauth::{ClientId, ClientSecret};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// <https://dev.twitch.tv/docs/cli/mock-api-command/#getting-an-access-token>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockData<T> {
    pub cursor: String,
    pub total: u64,
    pub data: Vec<T>,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    /// client_id
    pub ID: ClientId,
    /// client_secret
    pub Secret: ClientSecret,
    pub Name: String,
    pub IsExtension: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub email: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub broadcaster_type: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: u64,
    pub game_id: Game,
    pub game_name: Game,
    pub title: String,
    pub stream_language: String,
    pub delay: u32,
    pub is_branded_content: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "String")]
    pub string: String,
    #[serde(rename = "Valid")]
    pub valid: bool,
}
