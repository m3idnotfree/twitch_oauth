use asknothingx2_util::api;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{error, ClientId, ClientSecret, Error};

/// <https://dev.twitch.tv/docs/cli/mock-api-command/#getting-an-access-token>
#[derive(Debug, Serialize, Deserialize)]
pub struct UsersResponse {
    pub cursor: String,
    pub total: u64,
    pub data: Vec<User>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// client_id
    pub ID: ClientId,
    /// client_secret
    pub Secret: ClientSecret,
    pub Name: String,
    pub IsExtension: bool,
}

pub async fn get_users_info(port: Option<u16>) -> Result<UsersResponse, Error> {
    let mut base_url = Url::parse("http://localhost:8080").unwrap();
    if let Some(port) = port {
        base_url.set_port(Some(port)).unwrap();
    }

    base_url
        .path_segments_mut()
        .unwrap()
        .push("units")
        .push("clients");

    api::get(base_url)
        .await
        .send()
        .await
        .map_err(error::network::request)?
        .json()
        .await
        .map_err(error::validation::json)
}
