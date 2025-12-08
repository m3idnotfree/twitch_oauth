use std::{collections::HashMap, env, str::FromStr, sync::Arc, time::Instant};

use anyhow::{Context, Result};
use axum::{
    extract::{FromRef, Query, State},
    http::{header::SET_COOKIE, HeaderMap},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::{headers::Cookie, TypedHeader};
use tokio::sync::RwLock;
use tracing::{error, info, warn};
use twitch_oauth_token::{
    csrf::CsrfConfig,
    scope::{ChannelScopes, ChatScopes},
    AccessToken, OAuthCallbackQuery, RedirectUrl, TwitchOauth, UserAuth, UserToken, ValidateToken,
};

const COOKIE_NAME: &str = "OAuth-state";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    dotenvy::dotenv()?;

    let client_id = env::var("CLIENT_ID").context("CLIENT_ID environment variable not set")?;
    let client_secret =
        env::var("CLIENT_SECRET").context("CLIENT_SECRET environment variable not set")?;
    let redirect_uri =
        env::var("REDIRECT_URI").context("REDIRECT_URI environment variable not set")?;
    let port = env::var("PORT").context("PORT environment variable not set")?;

    info!(
        service = "twitch_oauth",
        action = "start_server",
        redirect_uri = %redirect_uri,
        "Starting Twitch OAuth server"
    );

    let oauth = TwitchOauth::new(client_id, client_secret)
        .set_redirect_uri(RedirectUrl::from_str(&redirect_uri)?)
        .set_csrf_config(CsrfConfig::new(0, 180));

    let state = AppState {
        oauth,
        user_tokens: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/auth/twitch", get(twitch_auth))
        .route("/auth/callback", get(auth_callback))
        .with_state(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr.clone())
        .await
        .context("Failed to bind to port 3000")?;

    info!(
        service = "twitch_oauth",
        action = "start_server",
        address = addr,
        "Server started successfully"
    );

    axum::serve(listener, app).await.context("Server error")?;

    Ok(())
}

async fn twitch_auth(State(client): State<TwitchOauth<UserAuth>>) -> impl IntoResponse {
    info!(
        service = "twitch_oauth",
        action = "generate_auth_url",
        "Generated OAuth URL"
    );

    let mut auth_url = client.authorization_url();
    auth_url
        .scopes_mut()
        .send_chat_message_as_user()
        .get_channel_emotes()
        .modify_channel_info();

    let (auth_url, state) = auth_url.url_with_state();

    let cookie =
        format!("{COOKIE_NAME}={state}; SameSite=Lax; HttpOnly; Secure; Path=/; Max-Age=1800");

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    (headers, Redirect::to(auth_url.as_str()))
}

async fn auth_callback(
    TypedHeader(cookies): TypedHeader<Cookie>,
    Query(callback): Query<OAuthCallbackQuery>,
    State(state): State<AppState>,
) -> String {
    let start_time = Instant::now();

    let stored_state = match cookies.get(COOKIE_NAME) {
        Some(state) => state,
        None => {
            error!(
                service = "twitch_oauth",
                action = "handle_callback",
                error_type = "missing_state_cookie",
                duration_ms = start_time.elapsed().as_millis(),
                "OAuth state cookie not found - possible CSRF attack or expired session"
            );
            return "Authentication failed: missing state cookie".to_string();
        }
    };

    if stored_state != callback.state {
        warn!(
            service = "twitch_oauth",
            action = "handle_callback",
            error_type = "state_mismatch",
            duration_ms = start_time.elapsed().as_millis(),
            "OAuth state mismatch - possible CSRF attack"
        );
        return "Authentication failed: invalid state".to_string();
    }

    info!(
        service = "twitch_oauth",
        action = "handle_callback",
        "Processing OAuth callback"
    );

    match process_callback(callback, state).await {
        Ok(user_info) => {
            info!(
                service = "twitch_oauth",
                action = "handle_callback",
                user_id = %user_info.user_id,
                username = %user_info.login,
                duration_ms = start_time.elapsed().as_millis(),
                "Successfully authenticated user"
            );
            format!("Successfully authenticated as {}", user_info.login)
        }
        Err(e) => {
            error!(
                service = "twitch_oauth",
                action = "handle_callback",
                error_type = "authentication_failed",
                error_message = %e,
                duration_ms = start_time.elapsed().as_millis(),
                "Failed to complete OAuth flow"
            );
            format!("Authentication failed: {:#}", e)
        }
    }
}

async fn process_callback(callback: OAuthCallbackQuery, state: AppState) -> Result<ValidateToken> {
    info!(
        service = "twitch_oauth",
        action = "exchange_code",
        "Exchanging authorization code for access token"
    );

    let token = state
        .oauth
        .user_access_token(callback.code, callback.state)
        .await
        .context("Failed to exchange authorization code for access token")?
        .user_token()
        .await
        .context("Failed to parse user token response")?;

    info!(
        service = "twitch_oauth",
        action = "exchange_code",
        token_type = token.token_type,
        expires_in = token.expires_in,
        "Successfully got user access token"
    );

    let user_info = validate_and_get_user_info(state.oauth, &token.access_token).await?;

    let mut user_tokens = state.user_tokens.write().await;
    user_tokens
        .entry(user_info.user_id.clone())
        .and_modify(|t| *t = token.clone())
        .or_insert(token);

    Ok(user_info)
}

async fn validate_and_get_user_info(
    oauth: TwitchOauth<UserAuth>,
    access_token: &AccessToken,
) -> Result<ValidateToken> {
    info!(
        service = "twitch_oauth",
        action = "validate_token",
        "Validating access token and retrieving user info"
    );

    let token = oauth
        .validate_access_token(access_token)
        .await
        .map_err(|e| {
            error!(
                service = "twitch_oauth",
                action = "validate_token",
                error_type = "validation_request_failed",
                error_message = %e,
                "Failed to validate access token"
            );

            e
        })
        .context("Failed to validate access token")?
        .validate_token()
        .await
        .context("Failed to parse token validation response")?;

    info!(
        service = "twitch_oauth",
        action = "validate_token",
        user_id = %token.user_id,
        username = %token.login,
        "Access token validated successfully"
    );

    Ok(token)
}

#[derive(Clone)]
struct AppState {
    pub oauth: TwitchOauth<UserAuth>,
    pub user_tokens: Arc<RwLock<HashMap<String, UserToken>>>,
}

impl FromRef<AppState> for TwitchOauth<UserAuth> {
    fn from_ref(input: &AppState) -> Self {
        input.oauth.clone()
    }
}
