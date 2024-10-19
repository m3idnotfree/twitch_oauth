use std::net::SocketAddr;

use anyhow::anyhow;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    RefreshToken, RevocationUrl, TokenUrl,
};
use url::Url;

use crate::{
    error::Error,
    request::{
        oauth_request, AuthrozationRequest, CodeTokenRequest, RefreshRequest, RevokeRequest,
        ValidateRequest, ValidateUrl,
    },
    types::{CodeState, GrantType, ResponseType, ServerStatus, Token, ValidateToken},
    Result,
};

const PORT: u16 = 60928;

#[derive(Debug)]
pub struct TwitchOauth {
    pub client_id: ClientId,
    client_secret: ClientSecret,
    auth_url: AuthUrl,
    token_url: TokenUrl,
    redirect_url: RedirectUrl,
    revoke_url: RevocationUrl,
    csrf_state: Option<CsrfToken>,
    validate_url: ValidateUrl,
    pub port: u16,
    access_token: Option<AccessToken>,
    refresh_token: Option<RefreshToken>,
    addr: SocketAddr,
}

impl Default for TwitchOauth {
    fn default() -> Self {
        Self {
            client_id: ClientId::new("".to_string()),
            client_secret: ClientSecret::new("".to_string()),
            auth_url: AuthUrl::new("https://id.twitch.tv/oauth2/authorize".to_string()).unwrap(),
            token_url: TokenUrl::new("https://id.twitch.tv/oauth2/token".to_string()).unwrap(),
            redirect_url: RedirectUrl::new(format!("http://localhost:{}", PORT)).unwrap(),
            revoke_url: RevocationUrl::new("https://id.twitch.tv/oauth2/revoke".to_string())
                .expect("Invalid revok URL"),
            csrf_state: None,
            port: PORT,
            validate_url: ValidateUrl::new("https://id.twitch.tv/oauth2/validate".to_string())
                .unwrap(),
            access_token: None,
            refresh_token: None,
            addr: SocketAddr::from(([127, 0, 0, 1], PORT)),
        }
    }
}

impl TwitchOauth {
    pub fn get_addr(&self) -> Result<SocketAddr> {
        let host = self.redirect_url.url().host_str().unwrap();
        if host != "localhost" {
            Err(Error::GetSocketAddrError(
                "redirect url not localhost".to_string(),
            ))
        } else {
            Ok(self.addr)
        }
    }
    pub fn set_client_id(mut self, client_id: &str) -> Self {
        self.client_id = ClientId::new(client_id.to_string());
        self
    }

    pub fn set_client_secret(mut self, client_secret: &str) -> Self {
        self.client_secret = ClientSecret::new(client_secret.to_string());
        self
    }

    pub fn set_redirect_uri(mut self, redir_url: &str) -> Result<Self> {
        let url = Url::parse(redir_url)?;

        if url.scheme() == "http" || url.scheme() == "https" {
            if let Some(port) = url.port() {
                self.port = port
            }

            self.redirect_url = RedirectUrl::new(url.to_string())?;
            Ok(self)
        } else {
            Err(Error::RedirectUrlError(anyhow!(
                "must set scheme http or https"
            )))
        }
    }

    pub fn set_access_token(mut self, access_token: AccessToken) -> Self {
        self.access_token = Some(access_token);
        self
    }
    pub fn set_refresh_token(mut self, refresh_token: RefreshToken) -> Self {
        self.refresh_token = Some(refresh_token);
        self
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        self.addr = SocketAddr::from(([127, 0, 0, 1], port));
        self
    }

    pub fn authorize_url(&mut self) -> AuthrozationRequest {
        let csrf_token = CsrfToken::new_random();
        self.csrf_state = Some(csrf_token.clone());

        AuthrozationRequest {
            auth_url: &self.auth_url,
            client_id: &self.client_id,
            redirect_url: &self.redirect_url,
            response_type: ResponseType::Code,
            scopes: Vec::new(),
            state: csrf_token,
        }
    }

    fn csrf_cmp(&self, state: CsrfToken) -> bool {
        self.csrf_state.as_ref().unwrap().secret() == state.secret()
    }

    pub async fn exchange_code(&self, code: AuthorizationCode) -> Result<Token> {
        let response = oauth_request(CodeTokenRequest {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            code,
            grant_type: GrantType::AuthorizationCode,
            token_url: &self.token_url,
            redirect_url: &self.redirect_url,
        })
        .await?;

        response.json()
    }

    pub async fn exchange_code_with_statuscode(&mut self, code_state: CodeState) -> Result<Token> {
        if !self.csrf_cmp(code_state.csrf_token.unwrap()) {
            self.csrf_state = None;
            Err(Error::CsrfTokenPartialEqError)
        } else {
            self.csrf_state = None;
            match code_state.state {
                ServerStatus::Timeout => Err(Error::TimeoutError("".to_string())),
                ServerStatus::Shutdown => Err(Error::GraceFulShutdown("".to_string())),
                ServerStatus::Recive => self.exchange_code(code_state.code.unwrap()).await,
            }
        }
    }

    pub async fn exchange_refresh_token(&self, refresh_token: &RefreshToken) -> Result<Token> {
        let response = oauth_request(RefreshRequest {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            grant_type: GrantType::RefreshToken,
            refresh_token,
            token_url: &self.token_url,
        })
        .await?;

        response.json()
    }
    pub async fn validate_token(&self, access_token: &AccessToken) -> Result<ValidateToken> {
        let response = oauth_request(ValidateRequest {
            access_token,
            validate_url: &self.validate_url,
        })
        .await?;

        response.json()
    }

    pub async fn revoke_token(&self, acces_token: &AccessToken) -> Result<()> {
        oauth_request(RevokeRequest {
            client_id: &self.client_id,
            revoke_url: &self.revoke_url,
            access_token: acces_token,
        })
        .await?;
        Ok(())
    }
}
