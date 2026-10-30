use asknothingx2_http::{
    Body, ClientService, HttpService, Request, client::Reqwest, config::preset,
};
use http::{HeaderValue, Uri, header::ACCEPT, uri::PathAndQuery};
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    ClientId, ClientSecret, Error,
    client::json,
    error::{BoxError, Operation},
    request::{APPLICATION_JSON, IntoRequest},
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
        let client = preset::danger_test(
            reqwest::Client::builder(),
            HeaderValue::from_static(concat!(
                env!("CARGO_PKG_NAME"),
                "-mock-api/",
                env!("CARGO_PKG_VERSION")
            )),
        )
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

    /// GET /units/categories
    pub async fn categories(&self) -> Result<Vec<MockCategory>, Error> {
        self.get("/units/categories").await
    }

    /// GET /units/clients
    pub async fn clients(&self) -> Result<Vec<MockClient>, Error> {
        self.get("/units/clients").await
    }

    /// GET /units/streams
    pub async fn streams(&self) -> Result<Vec<MockStream>, Error> {
        self.get("/units/streams").await
    }

    /// GET /units/subscriptions
    pub async fn subscriptions(&self) -> Result<Vec<MockSubscription>, Error> {
        self.get("/units/subscriptions").await
    }

    /// GET /units/tags
    pub async fn tags(&self) -> Result<Vec<MockTag>, Error> {
        self.get("/units/tags").await
    }

    /// GET /units/teams
    pub async fn teams(&self) -> Result<Vec<MockTeam>, Error> {
        self.get("/units/teams").await
    }

    /// GET /units/users
    pub async fn users(&self) -> Result<Vec<MockUser>, Error> {
        self.get("/units/users").await
    }

    /// GET /units/videos
    pub async fn videos(&self) -> Result<Vec<MockVideo>, Error> {
        self.get("/units/videos").await
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
pub struct MockCategory {
    pub id: String,
    pub name: String,
    pub box_art_url: String,
    pub igdb_id: String,
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
    #[serde(rename = "IsExtension")]
    pub is_extension: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockStream {
    pub id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub viewer_count: u64,
    pub started_at: String,
    pub is_mature: bool,
    pub tag_ids: Vec<String>,
    pub tags: Vec<String>,
    pub game_id: String,
    pub game_name: String,
    pub title: String,
    pub language: String,
    pub thumbnail_url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockSubscription {
    pub broadcaster_id: String,
    pub broadcaster_login: String,
    pub broadcaster_name: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub is_gift: bool,
    pub gifter_id: Option<String>,
    pub gifter_login: Option<String>,
    pub gifter_name: Option<String>,
    pub tier: String,
    pub plan_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockTag {
    pub id: String,
    pub is_auto: bool,
    pub tag_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockTeam {
    pub id: String,
    pub background_image_url: Option<String>,
    pub banner: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub info: String,
    pub thumbnail_url: String,
    pub team_name: String,
    pub team_display_name: String,
    pub users: Vec<MockTeamUser>,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockTeamUser {
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockUser {
    pub id: String,
    pub login: String,
    pub display_name: String,
    pub email: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub broadcaster_type: String,
    pub description: String,
    pub created_at: String,
    pub profile_image_url: String,
    pub offline_image_url: String,
    pub view_count: u64,
    #[serde(deserialize_with = "de::null_string")]
    pub game_id: Option<String>,
    #[serde(deserialize_with = "de::null_string")]
    pub game_name: Option<String>,
    pub title: String,
    pub stream_language: String,
    pub delay: u64,
    pub is_branded_content: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockVideo {
    pub id: String,
    pub stream_id: String,
    pub user_id: String,
    pub user_login: String,
    pub user_name: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
    pub published_at: String,
    pub viewable: String,
    pub view_count: u64,
    pub duration: String,
    pub language: String,
    pub muted_segments: Vec<MockMutedSegment>,
    #[serde(rename = "type")]
    pub kind: String,
    pub url: String,
    pub thumbnail_url: String,
}

#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct MockMutedSegment {
    pub offset: u64,
    pub duration: u64,
}

mod de {
    use serde::{Deserialize, Deserializer};
    /// `{"String": "", "Valid": false}`
    pub fn null_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NullString {
            #[serde(rename = "String")]
            string: String,
            #[serde(rename = "Valid")]
            valid: bool,
        }

        let value = NullString::deserialize(deserializer)?;
        Ok(value.valid.then_some(value.string))
    }
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
            .header(ACCEPT, APPLICATION_JSON)
            .body(Body::empty())?)
    }
}

#[cfg(test)]
mod tests {
    use super::de;

    #[derive(serde::Deserialize)]
    struct Game {
        #[serde(deserialize_with = "de::null_string")]
        game_id: Option<String>,
    }

    fn game_id(json: &str) -> Option<String> {
        serde_json::from_str::<Game>(json).unwrap().game_id
    }

    #[test]
    fn null_string_none() {
        assert_eq!(game_id(r#"{"game_id":{"String":"","Valid":false}}"#), None);
    }

    #[test]
    fn null_string_some() {
        assert_eq!(
            game_id(r#"{"game_id":{"String":"54742","Valid":true}}"#),
            Some("54742".to_string())
        );
    }
}
