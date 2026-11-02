use std::{env, str::FromStr};

use anyhow::{Context, Result};
use axum::{
    Router,
    extract::{Query, State},
    http::{StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Redirect},
    routing::get,
};
use axum_extra::{TypedHeader, headers::Cookie};
use twitch_oauth_token::{
    AuthorizationResponse, Client, ClientId, ClientSecret, ErrorCode, RedirectUri,
    State as OAuthState, UserAuth,
};

const STATE_COOKIE: &str = "__Host-oauth_state";
const STATE_MAX_AGE_SECS: u32 = 600;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let client = Client::new(
        ClientId::new(env::var("CLIENT_ID").context("CLIENT_ID is not set")?),
        ClientSecret::new(env::var("CLIENT_SECRET").context("CLIENT_SECRET is not set")?),
    )
    .with_redirect_uri(
        RedirectUri::from_str(&env::var("REDIRECT_URI").context("REDIRECT_URI is not set")?)
            .context("REDIRECT_URI is not a valid URL")?,
    );

    let app = Router::new()
        .route("/auth/twitch", get(login))
        .route("/auth/callback", get(callback))
        .with_state(client);

    let port = env::var("PORT").context("PORT is not set")?;
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn state_cookie(value: &str, max_age_secs: u32) -> String {
    format!(
        "{STATE_COOKIE}={value}; SameSite=Lax; HttpOnly; Secure; Path=/; Max-Age={max_age_secs}"
    )
}

async fn login(State(client): State<Client<UserAuth>>) -> impl IntoResponse {
    let mut request = client.authorization_url();
    request.scopes_mut().send_chat_message();

    let (auth_url, state) = request.url();

    (
        [(SET_COOKIE, state_cookie(state.secret(), STATE_MAX_AGE_SECS))],
        Redirect::to(auth_url.as_str()),
    )
}

async fn callback(
    State(client): State<Client<UserAuth>>,
    cookies: Option<TypedHeader<Cookie>>,
    Query(response): Query<AuthorizationResponse>,
) -> impl IntoResponse {
    let result = complete_login(&client, cookies, response).await;

    ([(SET_COOKIE, state_cookie("", 0))], result)
}

async fn complete_login(
    client: &Client<UserAuth>,
    cookies: Option<TypedHeader<Cookie>>,
    response: AuthorizationResponse,
) -> Result<String, (StatusCode, &'static str)> {
    let saved_state = cookies
        .as_ref()
        .and_then(|TypedHeader(cookies)| cookies.get(STATE_COOKIE))
        .filter(|value| !value.is_empty())
        .ok_or((StatusCode::BAD_REQUEST, "login expired, please try again"))?;

    let code = response
        .verify(&OAuthState::new(saved_state.to_string()))
        .map_err(|err| match err.error_code() {
            None => (StatusCode::BAD_REQUEST, "login expired, please try again"),
            Some(ErrorCode::AccessDenied) => (StatusCode::FORBIDDEN, "access was denied"),
            Some(code) => {
                eprintln!("authorization failed: {code:?}");
                (StatusCode::BAD_REQUEST, "login failed")
            }
        })?;

    let token = client.exchange_code(&code).await.map_err(|err| {
        eprintln!("failed to exchange authorization code: {err}");
        (StatusCode::BAD_GATEWAY, "login failed")
    })?;

    Ok(format!("token expires in {} seconds", token.expires_in))
}
