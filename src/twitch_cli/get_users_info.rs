use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub cursor: String,
    pub total: u64,
    pub data: Vec<User>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub ID: String,
    pub Secret: String,
    pub Name: String,
    pub IsExtension: bool,
}

pub async fn get_users_info(port: Option<u16>) -> Result<Users, reqwest::Error> {
    let mut base_url = Url::parse("http://localhost:8080").unwrap();
    if let Some(port) = port {
        base_url.set_port(Some(port)).unwrap();
    }

    base_url
        .path_segments_mut()
        .unwrap()
        .push("units")
        .push("clients");

    reqwest::get(base_url).await?.json().await
}
