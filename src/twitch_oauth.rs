use std::net::SocketAddr;

use anyhow::anyhow;
use asknothingx2_util::{
    api::api_request,
    oauth::{
        AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
        RefreshToken, RevocationUrl, TokenUrl, ValidateUrl,
    },
};
use url::Url;

use crate::{
    error::Error,
    request::{
        AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest, RefreshRequest,
        RevokeRequest, ValidateRequest,
    },
    types::{
        ClientCredentials, CodeState, GrantType, OauthResponse, ResponseType, ServerStatus, Token,
        ValidateToken,
    },
    Result,
};

const PORT: u16 = 60928;

#[derive(Debug)]
pub struct TwitchOauth {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub auth_url: AuthUrl,
    token_url: TokenUrl,
    redirect_url: RedirectUrl,
    revoke_url: RevocationUrl,
    csrf_state: Option<CsrfToken>,
    validate_url: ValidateUrl,
    pub port: u16,
    addr: SocketAddr,
    pub base_url: Url,
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
            addr: SocketAddr::from(([127, 0, 0, 1], PORT)),
            base_url: Url::parse("https://id.twitch.tv").unwrap(),
        }
    }
}

impl TwitchOauth {
    pub fn new(client_id: &str, client_secret: &str) -> Self {
        Self::default()
            .set_client_id(client_id)
            .set_client_secret(client_secret)
    }

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

    pub fn set_base_url(mut self, base_url: Url) -> Self {
        self.base_url = base_url;
        self
    }

    pub fn set_auth_url(mut self, auth_url: AuthUrl) -> Self {
        self.auth_url = auth_url;
        self
    }

    pub fn set_token_url(mut self, token_url: TokenUrl) -> Self {
        self.token_url = token_url;
        self
    }

    pub fn set_revoke_url(mut self, revoke_url: RevocationUrl) -> Self {
        self.revoke_url = revoke_url;
        self
    }

    pub fn set_validate_url(mut self, validate_url: ValidateUrl) -> Self {
        self.validate_url = validate_url;
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

    pub async fn exchange_code(&self, code: AuthorizationCode) -> Result<OauthResponse<Token>> {
        let response = api_request(CodeTokenRequest {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            code,
            grant_type: GrantType::AuthorizationCode,
            token_url: &self.token_url,
            redirect_url: &self.redirect_url,
        })
        .await?;

        Ok(OauthResponse::<Token>::new(
            response.status(),
            response.text().await?,
        ))
    }

    pub async fn exchange_code_with_statuscode(
        &mut self,
        code_state: CodeState,
    ) -> Result<OauthResponse<Token>> {
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

    pub async fn exchange_refresh_token(
        &self,
        refresh_token: &RefreshToken,
    ) -> Result<OauthResponse<Token>> {
        let response = api_request(RefreshRequest {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            grant_type: GrantType::RefreshToken,
            refresh_token,
            token_url: &self.token_url,
        })
        .await?;

        Ok(OauthResponse::<Token>::new(
            response.status(),
            response.text().await?,
        ))
    }
    pub async fn validate_token(
        &self,
        access_token: &AccessToken,
    ) -> Result<OauthResponse<ValidateToken>> {
        let response = api_request(ValidateRequest {
            access_token,
            validate_url: &self.validate_url,
        })
        .await?;

        Ok(OauthResponse::<ValidateToken>::new(
            response.status(),
            response.text().await?,
        ))
    }

    pub async fn revoke_token(&self, acces_token: &AccessToken) -> Result<()> {
        api_request(RevokeRequest {
            client_id: &self.client_id,
            revoke_url: &self.revoke_url,
            access_token: acces_token,
        })
        .await?;
        Ok(())
    }

    pub async fn client_credentials(&self) -> Result<OauthResponse<ClientCredentials>> {
        let response = api_request(ClientCredentialsRequest {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            grant_type: GrantType::ClientCredentials,
            token_url: &self.token_url,
        })
        .await?;

        Ok(OauthResponse::<ClientCredentials>::new(
            response.status(),
            response.text().await?,
        ))
    }
}
