use asknothingx2_util::{
    api::api_request,
    oauth::{AuthUrl, ClientId, ClientSecret, TokenUrl},
};
use serde::{Deserialize, Serialize};
use test_access_token::TestAccessToken;
use url::Url;

use crate::{
    request::ClientCredentialsRequest,
    scopes::ScopeBuilder,
    types::{ClientCredentials, GrantType},
    Error, Result,
};

mod test_access_token;

pub struct TwitchCli {
    client_id: ClientId,
    client_secret: ClientSecret,
    auth_url: AuthUrl,
    url: Url,
    base_url: Url,
}

impl Default for TwitchCli {
    fn default() -> Self {
        Self {
            client_id: ClientId::new("".to_string()),
            client_secret: ClientSecret::new("".to_string()),
            auth_url: AuthUrl::new("http://localhost:8080/auth/authorize".to_string()).unwrap(),
            url: Url::parse("http://localhost:8080/mock").unwrap(),
            base_url: Url::parse("http://localhost:8080").unwrap(),
        }
    }
}

impl TwitchCli {
    pub fn set_client_id(mut self, client_id: &str) -> Self {
        self.client_id = ClientId::new(client_id.to_string());
        self
    }

    pub fn set_client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = ClientSecret::new(client_secret.to_string());
        self
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.base_url.set_port(Some(port)).unwrap();
        self
    }

    pub async fn get_mock_users_info(&self) -> std::result::Result<Users, reqwest::Error> {
        let mut units_clients = self.base_url.clone();
        units_clients
            .path_segments_mut()
            .unwrap()
            .push("units")
            .push("clients");

        let client: Users = reqwest::get(units_clients).await?.json().await?;

        Ok(client)
    }

    pub fn get_access_token(&self) -> TestAccessToken {
        TestAccessToken {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            grant_type: GrantType::UserToken,
            user_id: "".to_string(),
            scopes: ScopeBuilder::default(),
            base_url: &self.base_url,
        }
    }
    pub async fn client_credentials(&self) -> Result<ClientCredentials> {
        let mut token_url = self.base_url.clone();
        token_url
            .path_segments_mut()
            .unwrap()
            .push("auth")
            .push("token");

        let response = api_request(ClientCredentialsRequest {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            grant_type: GrantType::ClientCredentials,
            token_url: &TokenUrl::new(token_url.to_string()).unwrap(),
        })
        .await?;

        response.json().await.map_err(Error::ReqwestError)
    }
}

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
