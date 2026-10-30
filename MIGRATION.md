## 5.x -> 6.0

### Update Type Names

```diff
- TwitchOauth
+ Client

- AuthrozationRequest
+ AuthorizationRequest

- DeviceAuthResponse
+ DeviceCodeResponse

- scope::InvalidScopeError
+ scope::ParseScopeError

- csrf::CsrfConfig
+ csrf::Config
```

### Constructors

`Client::new` takes `ClientId` and `ClientSecret`, and `DeviceAuth::new` takes `ClientId`.

```diff
- let oauth = TwitchOauth::new(id, secret);
+ let oauth = Client::new(ClientId::from(id), ClientSecret::from(secret));

- let oauth = TwitchOauth::from_credentials(client_id, client_secret);
+ let oauth = Client::new(client_id, client_secret);
```

```diff
- let device = TwitchOauth::device_auth(client_id);
+ let device = DeviceAuth::new(client_id);
```

### Construction-time Settings

Settings fixed at construction go through `ClientBuilder`. `secret_key` is renamed to `csrf_key`.
`build`, `http_client` and `service` finish the builder and return a `Client`.

```diff
- let oauth = TwitchOauth::new(id, secret)
-     .with_csrf_config(config)
-     .with_redirect_uri(url)
-     .with_secret_key(key);
+ let oauth = Client::builder(ClientId::from(id), ClientSecret::from(secret))
+     .csrf_config(config)
+     .csrf_key(key)
+     .build()
+     .with_redirect_uri(url);
```

`set_csrf_config` is only on `Client<UserAuth>`.

```diff
- oauth.set_secret_key(key);
+ oauth.set_csrf_key(key);
```

```diff
- let uri = oauth.get_redirect_uri();
+ let uri = oauth.redirect_uri();
```

### HTTP Client

The global client is removed. Each `Client` and `DeviceAuth` has its own HTTP client.
`Client::get_client` is removed.

```diff
- client::setup(http_client)?;
- let oauth = TwitchOauth::new(id, secret);
+ let oauth = Client::builder(ClientId::from(id), ClientSecret::from(secret))
+     .http_client(http_client);
```

```diff
- let oauth = TwitchOauth::new(id, secret).with_client(http_client);
+ let oauth = Client::builder(ClientId::from(id), ClientSecret::from(secret))
+     .http_client(http_client);
```

### Device Code Flow

Settings go through `DeviceAuthBuilder`. `with_device_url` and `with_token_url` are removed.
`build`, `http_client` and `service` finish the builder and return a `DeviceAuth`.

```diff
- let device = DeviceAuth::new(client_id)
-     .with_scopes(scopes)
-     .with_client(http_client);
+ let device = DeviceAuth::builder(client_id)
+     .scopes(scopes)
+     .http_client(http_client);

- let resp = device.request().await?;
- let token = device.poll(resp).await?;
+ let resp = device.device_code().await?;
+ let token = device.wait_for_token(resp).await?;
```

`DeviceCodeResponse::created_at` is no longer public.
Requests are sent as `application/x-www-form-urlencoded` instead of `multipart/form-data`.

### Tokens

`refresh_access_token` and `exchange_code` take references.

```diff
- let token = oauth.refresh_access_token(refresh_token).await?;
+ let token = oauth.refresh_access_token(&refresh_token).await?;

- let token = oauth.exchange_code(code, state).await?;
+ let token = oauth.exchange_code(&code, &state).await?;
```

`UserToken::created_at` and `UserToken::is_expired` are removed.

```diff
pub struct UserToken {
    pub access_token: AccessToken,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: RefreshToken,
    pub scope: Vec<Scope>,
-   pub created_at: i64,
}
```

```diff
{
    "access_token": "...",
    "expires_in": 14124,
    "token_type": "bearer",
    "refresh_token": "...",
    "scope": ["chat:read"],
-   "created_at": 1761300000
}
```

`TokenInfo::login` and `TokenInfo::user_id` are `Option<String>`. Twitch returns `null` for them
with an App Access Token.

```diff
pub struct TokenInfo {
    pub client_id: ClientId,
    pub scopes: Vec<Scope>,
    pub expires_in: u64,
-   pub login: String,
-   pub user_id: String,
+   pub login: Option<String>,
+   pub user_id: Option<String>,
}
```

### Scope

`Scope` no longer derefs to `str`, and is `#[non_exhaustive]`.

```diff
- let name: &str = &scope;
+ let name: &str = scope.as_str();

match scope {
    Scope::ChatRead => {}
+   _ => {}
}
```

### CSRF

`csrf` re-exports only `Config`, `Error` and `generate_secret_key`.
Use `asknothingx2_util::oauth::signed_token` directly.

### URL Types

The `AuthUrl`, `DeviceUrl`, `RevocationUrl`, `TokenUrl` and `ValidateUrl` re-exports are removed.

### Oneshot Server

The `oneshot` feature and the `oneshot` re-export are removed. Use `asknothingx2-util` directly.

```diff
- twitch_oauth_token = { version = "5.0", features = ["oneshot"] }
+ twitch_oauth_token = "6.0"
+ asknothingx2-util = { version = "0.7", features = ["oauth-server"] }
```

### Mock API

The `test` feature is renamed to `mock-api`, and the `test_oauth` module to `mock_api`.

```diff
- twitch_oauth_token = { version = "5.0", features = ["test"] }
+ twitch_oauth_token = { version = "6.0", features = ["mock-api"] }
```

Send requests to the mock API with `MockApiLayer`, and get the mock data with `MockUnits`.

```rust
let units = MockUnits::localhost(8080);

let client = units.clients().await?.remove(0);
let user = units.users().await?.remove(0);

// let categories = units.categories().await?;
// let streams = units.streams().await?;
// let subscriptions = units.subscriptions().await?;
// let tags = units.tags().await?;
// let teams = units.teams().await?;
// let videos = units.videos().await?;

let oauth = Client::builder(client.id, client.secret)
    .layer(MockApiLayer::localhost(8080))
    .http_client(reqwest::Client::new());

let app_token = oauth.app_access_token().await?;

let mut request = oauth.mock_user_token(&user.id);
request.scopes_mut().update_user();
let user_token = request.send().await?;
```

## 4.x -> 5.0

`tracing` is now an optional feature. Enable it to keep the `DeviceAuth::poll` events.

```diff
- twitch_oauth_token = "4.4"
+ twitch_oauth_token = { version = "5.0", features = ["tracing"] }
```

### Error Handling

```diff
- error.is_request_error()
+ error.kind() == ErrorKind::Request

- error.is_decode()
+ error.kind() == ErrorKind::Parse

- error.is_oauth_error()
+ matches!(error.kind(), ErrorKind::Api | ErrorKind::Csrf)

- error.is_device_code_error()
+ matches!(error.kind(), ErrorKind::DeviceCodeExpired | ErrorKind::Api)

- error.status_code()
+ error.status()
```

- `Error::raw` is removed. The response body is no longer kept.
- `Error::message` returns only the message from Twitch.

### Client Setup

`client::setup` returns `client::Error`.

```diff
- fn init(http_client: reqwest::Client) -> Result<(), twitch_oauth_token::Error> {
+ fn init(http_client: reqwest::Client) -> Result<(), twitch_oauth_token::client::Error> {
      client::setup(http_client)?;
      Ok(())
  }
```

### Update Type Names

```diff
- use twitch_oauth_token::csrf::TokenError;
+ use twitch_oauth_token::csrf::Error;
```

## 3.x -> 4.0

### Update Feature Flags

```diff
[dependencies]
- twitch_oauth_token = { version = "3.1", features = ["oneshot-server"] }
+ twitch_oauth_token = { version = "4.0", features = ["oneshot"] }
```

### Update Imports

```diff
- use twitch_oauth_token::{oneshot_server, OAuthCallbackQuery};
+ use twitch_oauth_token::{oneshot, AuthCallback};
```

### Update Method Calls

#### Exchange authorization code

```diff
- let resp = oauth.user_access_token(code, state).await?;
- let token = resp.user_token().await?;
+ let token = oauth.exchange_code(code, state).await?;

```

#### Builder methods

```diff
- let oauth = TwitchOauth::new(id, secret).set_redirect_uri(url);
+ let oauth = TwitchOauth::new(id, secret).with_redirect_uri(url);
```

#### Response handling (no more wrapper)

```diff
// App token
- let resp = oauth.app_access_token().await?;
- let token = resp.app_token().await?;
+ let token = oauth.app_access_token().await?;

// Refresh token
- let resp = oauth.refresh_access_token(refresh_token).await?;
- let token = resp.user_token().await?;
+ let new_token = oauth.refresh_access_token(refresh_token).await?;

// Validate token
- let resp = oauth.validate_access_token(&access_token).await?;
- let token_info = resp.validate_token();
+ let token_info = oauth.validate_access_token(&access_token).await?;

// Revoke token
- let _resp = oauth.revoke_access_token(&access_token).await?;
+ oauth.revoke_access_token(&access_token).await?;
```

#### oneshot server

```diff
- use twitch_oauth_token::oneshot_server::oneshot_server;
- let callback = oneshot_server("127.0.0.1:3000", Duration::from_secs(120)).await?;
+ use twitch_oauth_token::{oneshot, AuthCallback};
+ let config = oneshot::Config::new()
+     .with_port(3000)
+     .with_duration(Duration::from_secs(120));
+ let callback: AuthCallback = oneshot::listen(config).await?;
```

### Update Type Names

```diff
- CodeTokenRequest
+ ExchangeCodeRequest

- OAuthCallbackQuery
+ AuthCallback

- ValidateToken
+ TokenInfo
```

### Error Handling

```diff
- error.is_validation_error()
- error.is_response_parsing_error()
+ error.is_decode()

- error.is_retryable()
- error.is_network_error()
+ error.is_request_error
```

### Client Setup

```diff
- client::setup(|preset| {
-     Ok(preset
-         .timeouts(Duration::from_secs(60), Duration::from_secs(30))
-         .connections(10, Duration::from_secs(90))
-         .default_headers(|headers| {
-             headers
-                 .accept_json()
-                 .content_type_json()
-                 .header_str(HeaderName::from_str("Custom-Header")?, "value")?;
-             Ok(())
-         })?
-         .user_agent("MyApp/1.0"))
- })?;
+ use asknothingx2_util::api::preset;
+
+ let mut preset = preset::authentication("MyApp/1.0");
+ preset
+     .timeouts(Duration::from_secs(60), Duration::from_secs(30))
+     .connections(10, Duration::from_secs(90));
+
+ preset
+     .default_headers_mut()
+     .accept_json()
+     .content_type_json();
+     .header_str(HeaderName::from_str("Custom-Header")?, "value")?;
+
+ client::setup(preset.build()?)?;
```
