use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    reqwest::{async_http_client, http_client},
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RequestTokenError, Scope,
    StandardErrorResponse, StandardTokenResponse, TokenUrl,
};

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
                ),
        )
    }

    pub fn pkce() -> (PkceCodeChallenge, PkceCodeVerifier) {
        PkceCodeChallenge::new_random_sha256()
    }

    pub fn auth_url(
        &self,
        scopes: Vec<&str>,
        pkce: PkceCodeChallenge,
    ) -> (reqwest::Url, oauth2::CsrfToken) {
        let scopes = scopes.iter().map(|value| Scope::new(value.to_string()));
        self.0
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .set_pkce_challenge(pkce)
            .url()
    }

    pub async fn exchange_token_async(
        &self,
        auth_code: AuthorizationCode,
        pkce_verifier: PkceCodeVerifier,
        // ) -> oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>
    ) -> Result<
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    > {
        self.0
            .exchange_code(auth_code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(async_http_client)
            .await
    }

    pub async fn exchange_token(
        &self,
        auth_code: AuthorizationCode,
        pkce_verifier: PkceCodeVerifier,
    ) -> Result<
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    > {
        self.0
            .exchange_code(auth_code)
            .set_pkce_verifier(pkce_verifier)
            .request(http_client)
    }
}

// #[derive(Debug, thiserror::Error)]
// pub enum OauthError {
//     #[error("timeout '{0}'")]
//     AcceptTimeout(#[from] tokio::time::error::Elapsed),
//     #[error("can't read stream '{0}'")]
//     ReadStream(String),
// }
