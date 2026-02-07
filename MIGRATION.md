### 1. Update Feature Flags

```diff
[dependencies]
- twitch_oauth_token = { version = "3.1", features = ["oneshot-server"] }
+ twitch_oauth_token = { version = "4.0", features = ["oneshot"] }
```

### 2. Update Imports

```diff
- use twitch_oauth_token::{oneshot_server, OAuthCallbackQuery};
+ use twitch_oauth_token::{oneshot, AuthCallback};
```

### 3. Update Method Calls

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

### 4. Update Type Names

```diff
- CodeTokenRequest
+ ExchangeCodeRequest

- OAuthCallbackQuery
+ AuthCallback

- ValidateToken
+ TokenInfo
```

### 5. Error Handling

```diff
- error.is_validation_error()
- error.is_response_parsing_error()
+ error.is_decode()

- error.is_retryable()
- error.is_network_error()
+ error.is_request_error
```

### 5. Client Setup

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
