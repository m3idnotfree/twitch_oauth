use asknothingx2_http::{Body, ClientService, HttpService, Request, client::Reqwest};
use asknothingx2_util::api::{mime_type::Application, preset};
use http::{Uri, header::ACCEPT, uri::PathAndQuery};
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    ClientId, ClientSecret, Error,
    client::json,
    error::{BoxError, Operation},
    request::IntoRequest,
};

/// <https://dev.twitch.tv/docs/cli/mock-api-command/#getting-an-access-token>
#[derive(Debug, Clone)]
pub struct MockUnits {
    base: Uri,
    svc: ClientService,
}

impl MockUnits {
    pub fn localhost(port: u16) -> Self {
        Self::new(super::localhost(port))
    }

    pub fn new(base: Uri) -> Self {
        let client = preset::testing(concat!(
            env!("CARGO_PKG_NAME"),
            "-mock-api/",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .expect("failed to build mock API http client");

        Self {
            base,
            svc: ClientService::new(Reqwest::new(client)),
        }
    }

    pub fn with_service<S: HttpService>(mut self, service: S) -> Self {
        self.svc = ClientService::new(service);
        self
    }

    /// GET /units/clients
    pub async fn clients(&self) -> Result<Vec<MockClient>, Error> {
        self.get("/units/clients").await
    }

    /// GET /units/users
    pub async fn users(&self) -> Result<Vec<MockUser>, Error> {
        self.get("/units/users").await
    }

    async fn get<T: DeserializeOwned>(&self, path: &'static str) -> Result<Vec<T>, Error> {
        let units: Units<T> = json(
            self.svc.clone(),
            UnitsRequest {
                base: &self.base,
                path,
            },
        )
        .await?;

        Ok(units.data)
    }
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockClient {
    #[serde(rename = "ID")]
    pub id: ClientId,
    #[serde(rename = "Secret")]
    pub secret: ClientSecret,
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockUser {
    pub id: String,
    pub login: String,
    pub display_name: String,
}

#[derive(Deserialize)]
struct Units<T> {
    data: Vec<T>,
}

struct UnitsRequest<'a> {
    base: &'a Uri,
    path: &'static str,
}

impl IntoRequest for UnitsRequest<'_> {
    const OPERATION: Operation = Operation::MockUnits;

    fn into_request(self) -> Result<Request, BoxError> {
        let mut parts = self.base.clone().into_parts();
        parts.path_and_query = Some(PathAndQuery::from_static(self.path));

        Ok(http::Request::get(Uri::from_parts(parts)?)
            .header(ACCEPT, Application::Json)
            .body(Body::empty())?)
    }
}
