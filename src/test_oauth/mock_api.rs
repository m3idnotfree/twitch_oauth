use asknothingx2_util::api;
use serde::de::DeserializeOwned;
use url::Url;

use crate::{error, Error};

use super::response::{Client, MockData, User};

pub struct MockApiUnits {
    port: u16,
    url: Url,
    client: reqwest::Client,
}

impl MockApiUnits {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self.url.set_port(Some(port)).unwrap();
        self
    }

    pub fn set_client(mut self, client: reqwest::Client) -> Self {
        self.client = client;
        self
    }

    pub async fn get_clients(&self) -> Result<MockData<Client>, Error> {
        let mut url = self.url.clone();

        url.path_segments_mut().unwrap().push("clients");

        self.send_json(url).await
    }

    pub async fn get_users(&self) -> Result<MockData<User>, Error> {
        let mut url = self.url.clone();

        url.path_segments_mut().unwrap().push("users");

        self.send_json(url).await
    }

    async fn send_json<T: DeserializeOwned>(&self, url: Url) -> Result<MockData<T>, Error> {
        self.client
            .get(url)
            .send()
            .await
            .map_err(error::network::request)?
            .json()
            .await
            .map_err(error::response::json)
    }
}

impl Default for MockApiUnits {
    fn default() -> Self {
        Self {
            port: 8080,
            url: Url::parse("http://localhost:8080/units").unwrap(),
            client: api::preset::testing("twitch-oauth-test/1.0")
                .build_client()
                .unwrap(),
        }
    }
}
