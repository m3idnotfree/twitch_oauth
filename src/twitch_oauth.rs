use asknothingx2_util::{
    api::api_request,
    oauth::{
        AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
        RefreshToken, RevocationUrl, TokenUrl, ValidateUrl,
    },
};

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

const BASE_URL: &str = "https://id.twitch.tv/oauth2";
const AUTH: &str = "authorize";
const TOKEN: &str = "token";
const REVOKE: &str = "revoke";
const VALIDATE: &str = "validate";

#[derive(Debug)]
pub struct TwitchOauth {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    /// default: http://localhost:60928
    pub redirect_url: RedirectUrl,
    csrf_state: Option<CsrfToken>,
    #[cfg(feature = "test")]
    pub test_url: Option<String>,
}

impl TwitchOauth {
    pub fn new<T: Into<String>>(client_id: T, client_secret: T) -> Self {
        Self {
            client_id: ClientId::new(client_id.into()),
            client_secret: ClientSecret::new(client_secret.into()),
            redirect_url: RedirectUrl::new(format!("http://localhost:{}", PORT)).unwrap(),
            csrf_state: None,
            #[cfg(feature = "test")]
            test_url: None,
        }
    }

    pub fn set_client_id<T: Into<String>>(mut self, client_id: T) -> Self {
        self.client_id = ClientId::new(client_id.into());
        self
    }

    pub fn set_client_secret<T: Into<String>>(mut self, client_secret: T) -> Self {
        self.client_secret = ClientSecret::new(client_secret.into());
        self
    }

    pub fn set_redirect_uri<T: Into<String>>(mut self, redir_url: T) -> Result<Self> {
        self.redirect_url = RedirectUrl::new(redir_url.into())?;
        Ok(self)
    }

    pub fn get_auth_url(&self) -> AuthUrl {
        #[cfg(feature = "test")]
        if let Some(url) = &self.test_url {
            return AuthUrl::new(url.to_string()).unwrap();
        }

        AuthUrl::new(format!("{BASE_URL}/{AUTH}")).unwrap()
    }

    fn get_token_url(&self) -> TokenUrl {
        TokenUrl::new(format!("{BASE_URL}/{TOKEN}")).unwrap()
    }

    fn get_revoke_url(&self) -> RevocationUrl {
        RevocationUrl::new(format!("{BASE_URL}/{REVOKE}")).unwrap()
    }

    fn get_validate_url(&self) -> ValidateUrl {
        ValidateUrl::new(format!("{BASE_URL}/{VALIDATE}")).unwrap()
    }

    pub fn authorize_url(&mut self) -> AuthrozationRequest {
        let csrf_token = CsrfToken::new_random();
        self.csrf_state = Some(csrf_token.clone());

        AuthrozationRequest::new(
            self.get_auth_url(),
            self.client_id.clone(),
            self.redirect_url.clone(),
            ResponseType::Code,
            csrf_token,
        )
    }

    fn csrf_cmp(&self, state: CsrfToken) -> bool {
        self.csrf_state.as_ref().unwrap().secret() == state.secret()
    }

    pub async fn exchange_code(&self, code: AuthorizationCode) -> Result<OauthResponse<Token>> {
        let response = api_request(CodeTokenRequest::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            code,
            GrantType::AuthorizationCode,
            self.get_token_url(),
            self.redirect_url.clone(),
        ))
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
        refresh_token: RefreshToken,
    ) -> Result<OauthResponse<Token>> {
        let response = api_request(RefreshRequest::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::RefreshToken,
            refresh_token,
            self.get_token_url(),
        ))
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
        let response = api_request(ValidateRequest::new(
            access_token.clone(),
            self.get_validate_url(),
        ))
        .await?;

        Ok(OauthResponse::<ValidateToken>::new(
            response.status(),
            response.text().await?,
        ))
    }

    pub async fn revoke_token(&self, access_token: AccessToken) -> Result<()> {
        api_request(RevokeRequest::new(
            access_token,
            self.client_id.clone(),
            self.get_revoke_url(),
        ))
        .await?;
        Ok(())
    }

    pub async fn client_credentials(&self) -> Result<OauthResponse<ClientCredentials>> {
        let response = api_request(ClientCredentialsRequest::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            GrantType::ClientCredentials,
            self.get_token_url(),
        ))
        .await?;

        Ok(OauthResponse::<ClientCredentials>::new(
            response.status(),
            response.text().await?,
        ))
    }
}
