use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    http::HeaderMap,
    reqwest::{async_http_client, http_client},
    url::Url,
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, ConfigurationError, CsrfToken,
    EmptyExtraTokenFields, HttpRequest, HttpResponse, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, RefreshToken, RequestTokenError, RevocationErrorResponseType, RevocationRequest,
    RevocationUrl, Scope, StandardErrorResponse, StandardRevocableToken, StandardTokenResponse,
    TokenUrl,
};
use reqwest::{header::AUTHORIZATION, Method};

use crate::Token;

pub type RequestTokenResult = Result<
    StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    RequestTokenError<
        oauth2::reqwest::Error<reqwest::Error>,
        StandardErrorResponse<BasicErrorResponseType>,
    >,
>;

#[derive(Clone)]
pub struct TwitchOauth(BasicClient);

impl TwitchOauth {
    pub fn new(client_id: &str, client_secret: &str, redirect: &str) -> Self {
        let client_id = ClientId::new(client_id.to_string());
        let client_secret = ClientSecret::new(client_secret.to_string());

        let auth_url = AuthUrl::new("https://id.twitch.tv/oauth2/authorize".to_string()).unwrap();
        let token_url = TokenUrl::new("https://id.twitch.tv/oauth2/token".to_string()).unwrap();

        Self(
            BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
                .set_redirect_uri(
                    RedirectUrl::new(redirect.to_string()).expect("Invalid redirect URL"),
                )
                .set_revocation_uri(
                    RevocationUrl::new("https://id.twitch.tv/oauth2/revoke".to_string())
                        .expect("Invalid revok URL"),
                ),
        )
    }

    pub fn pkce() -> (PkceCodeChallenge, PkceCodeVerifier) {
        PkceCodeChallenge::new_random_sha256()
    }

    pub fn auth_url(&self, scopes: Vec<String>, pkce: PkceCodeChallenge) -> (Url, CsrfToken) {
        let scopes = scopes.into_iter().map(Scope::new);
        self.0
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .set_pkce_challenge(pkce)
            .url()
    }

    pub async fn exchange_token(
        &self,
        auth_code: AuthorizationCode,
        pkce_verifier: PkceCodeVerifier,
    ) -> RequestTokenResult {
        self.0
            .exchange_code(auth_code)
            .set_pkce_verifier(pkce_verifier)
            .request(http_client)
    }

    pub async fn exchange_token_async(
        &self,
        auth_code: AuthorizationCode,
        pkce_verifier: PkceCodeVerifier,
        // ) -> oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>
    ) -> RequestTokenResult {
        self.0
            .exchange_code(auth_code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
    }

    pub fn refresh_token(&self, token: &Token) -> RequestTokenResult {
        self.0
            .exchange_refresh_token(&RefreshToken::new(token.refresh_token.clone()))
            .request(http_client)
    }

    pub async fn refresh_token_async(&self, token: &Token) -> RequestTokenResult {
        self.0
            .exchange_refresh_token(&RefreshToken::new(token.refresh_token.clone()))
            .request_async(async_http_client)
            .await
    }

    pub fn revoke_token(
        &self,
        token: &Token,
    ) -> Result<
        RevocationRequest<
            '_,
            StandardRevocableToken,
            StandardErrorResponse<RevocationErrorResponseType>,
        >,
        ConfigurationError,
    > {
        self.0
            .revoke_token(oauth2::StandardRevocableToken::AccessToken(
                AccessToken::new(token.access_token.to_string()),
            ))
    }

    pub fn validate_token(
        &self,
        token: &Token,
    ) -> Result<HttpResponse, oauth2::reqwest::Error<reqwest::Error>> {
        let auth_value = format!("OAuth {}", token.access_token);

        http_client(self.validate_request(auth_value))
    }

    pub async fn validate_token_async(
        &self,
        token: &Token,
    ) -> Result<HttpResponse, oauth2::reqwest::Error<reqwest::Error>> {
        let auth_value = format!("OAuth {}", token.access_token);

        async_http_client(self.validate_request(auth_value)).await
    }

    fn validate_request(&self, access_token: String) -> HttpRequest {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, access_token.parse().unwrap());

        HttpRequest {
            url: "https://id.twitch.tv/oauth2/validate"
                .parse::<Url>()
                .unwrap(),
            method: Method::GET,
            headers,
            body: Vec::new(),
        }
    }
}

// #[derive(Debug, thiserror::Error)]
// pub enum OauthError {
//     #[error("timeout '{0}'")]
//     AcceptTimeout(#[from] tokio::time::error::Elapsed),
//     #[error("can't read stream '{0}'")]
//     ReadStream(String),
// }
