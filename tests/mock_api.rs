//! ```sh
//! cargo test --features mock-api --test mock_api -- --ignored
//! ```

use std::{
    net::TcpListener,
    process::{Child, Command},
    sync::{Arc, Weak},
    time::Duration,
};

use anyhow::{Context, Result, bail};
use tokio::{
    net::TcpStream,
    sync::Mutex,
    time::{Instant, sleep},
};
use twitch_oauth_token::{
    Client, Scope,
    mock_api::{MockApiLayer, MockClient, MockUnits, MockUser},
};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(10);

static SERVER: Mutex<Weak<Server>> = Mutex::const_new(Weak::new());

struct Server {
    child: Child,
    port: u16,
}

impl Server {
    async fn shared() -> Result<Arc<Self>> {
        let mut shared = SERVER.lock().await;

        if let Some(server) = shared.upgrade() {
            return Ok(server);
        }

        let server = Arc::new(Self::start().await?);
        *shared = Arc::downgrade(&server);

        Ok(server)
    }

    async fn start() -> Result<Self> {
        let port = TcpListener::bind(("127.0.0.1", 0))?.local_addr()?.port();

        let child = Command::new("twitch")
            .args(["mock-api", "start", "-p", &port.to_string()])
            .env("TWITCH_DISABLE_UPDATE_CHECKS", "true")
            .spawn()
            .context("failed to run `twitch`. is the Twitch CLI installed?")?;

        let mut server = Self { child, port };

        let deadline = Instant::now() + DEFAULT_TIMEOUT;

        while !is_listening(port).await {
            if let Some(status) = server.child.try_wait()? {
                bail!("`twitch` exited with {status} before it listened on port {port}");
            }

            if Instant::now() >= deadline {
                bail!("`twitch` did not listen on port {port} within {DEFAULT_TIMEOUT:?}");
            }

            sleep(Duration::from_millis(100)).await;
        }

        Ok(server)
    }

    fn units(&self) -> MockUnits {
        MockUnits::localhost(self.port)
    }

    async fn client(&self) -> Result<MockClient> {
        self.units()
            .clients()
            .await?
            .into_iter()
            .next()
            .context("no mock clients, run `twitch mock-api generate`")
    }

    async fn oauth(&self) -> Result<Client> {
        let client = self.client().await?;

        Ok(Client::builder(client.id, client.secret)
            .layer(MockApiLayer::localhost(self.port))
            .http_client(reqwest::Client::new()))
    }

    async fn user(&self) -> Result<MockUser> {
        self.units()
            .users()
            .await?
            .into_iter()
            .next()
            .context("no mock users, run `twitch mock-api generate`")
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

async fn is_listening(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port)).await.is_ok()
}

#[tokio::test]
#[ignore = "needs the Twitch CLI"]
async fn deserializting_units() -> Result<()> {
    let server = Server::shared().await?;
    let units = server.units();

    let categories = units.categories().await?;
    assert!(!categories.is_empty());

    let clients = units.clients().await?;
    assert!(!clients.is_empty());

    let streams = units.streams().await?;
    assert!(!streams.is_empty());

    let subscriptions = units.subscriptions().await?;
    assert!(!subscriptions.is_empty());

    let tags = units.tags().await?;
    assert!(!tags.is_empty());

    let teams = units.teams().await?;
    assert!(!teams.is_empty());

    let users = units.users().await?;
    assert!(!users.is_empty());

    let videos = units.videos().await?;
    assert!(!videos.is_empty());

    Ok(())
}

#[tokio::test]
#[ignore = "needs the Twitch CLI"]
async fn app_access_token() -> Result<()> {
    let server = Server::shared().await?;
    let oauth = server.oauth().await?;

    let token = oauth.app_access_token().await?;

    assert!(!token.access_token.secret().is_empty());
    Ok(())
}

#[tokio::test]
#[ignore = "needs the Twitch CLI"]
async fn mock_user_token() -> Result<()> {
    let server = Server::shared().await?;
    let oauth = server.oauth().await?;
    let user = server.user().await?;

    let mut req = oauth.mock_user_token(&user.id);
    req.scopes_mut().extend([Scope::UserEdit]);
    let token = req.send().await?;

    assert!(!token.access_token.secret().is_empty());
    assert_eq!(token.scope, [Scope::UserEdit]);
    Ok(())
}

#[tokio::test]
#[ignore = "needs the Twitch CLI"]
async fn validate_mock_user_token() -> Result<()> {
    let server = Server::shared().await?;
    let oauth = server.oauth().await?;
    let user = server.user().await?;

    let mut request = oauth.mock_user_token(&user.id);
    request.scopes_mut().extend([Scope::UserEdit]);
    let token = request.send().await?;

    let info = oauth.validate_access_token(&token.access_token).await?;

    assert_eq!(info.user_id.as_deref(), Some(user.id.as_str()));
    assert_eq!(info.scopes, [Scope::UserEdit]);
    Ok(())
}
