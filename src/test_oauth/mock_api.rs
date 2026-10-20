use asknothingx2_http::{Body, ClientService, client::Reqwest};
use asknothingx2_util::api;
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    Error, error,
    test_oauth::{
        USER_AGENT,
        response::{Client, MockData, User},
    },
};

#[derive(Debug, Clone)]
pub struct MockApiUnits {
    port: u16,
    url: Url,
    client: reqwest::Client,
    svc: ClientService,
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
        self.svc = ClientService::new(Reqwest::new(client.clone()));
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
        let req = http::Request::get(url.as_str())
            .body(Body::empty())
            .map_err(|e| error::build(error::Operation::MockApiUnits, e))?;
        let resp = self
            .svc
            .clone()
            .execute(req)
            .await
            .map_err(|e| error::request(error::Operation::MockApiUnits, e.into_inner()))?;
        crate::oauth::read_json(error::Operation::MockApiUnits, resp).await
    }
}

impl Default for MockApiUnits {
    fn default() -> Self {
        Self {
            port: 8080,
            url: Url::parse("http://localhost:8080/units").unwrap(),
            client: api::preset::testing(USER_AGENT).build().unwrap(),
            svc: ClientService::new(Reqwest::new(
                api::preset::testing(USER_AGENT).build().unwrap(),
            )),
        }
    }
}
