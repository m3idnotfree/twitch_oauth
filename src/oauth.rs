use std::{fmt, marker::PhantomData};

use asknothingx2_util::{
    api::api_request,
    oauth::{
        AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
        RefreshToken, RevocationUrl, TokenUrl, ValidateUrl,
    },
};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

use crate::{
    error::ErrorResponse,
    request::{
        AuthrozationRequest, ClientCredentialsRequest, CodeTokenRequest, RefreshRequest,
        RevokeRequest, ValidateRequest,
    },
    types::{
        ClientCredentials, CodeState, GrantType, ResponseType, ServerStatus, Token, ValidateToken,
    },
    Error, Result,
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
    #[cfg(feature = "test")]
    pub test_url: Option<String>,
}

impl TwitchOauth {
    pub fn new<T: Into<String>>(client_id: T, client_secret: T) -> Self {
        Self {
            client_id: ClientId::new(client_id.into()),
            client_secret: ClientSecret::new(client_secret.into()),
            redirect_url: RedirectUrl::new(format!("http://localhost:{}", PORT)).unwrap(),
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

    pub fn authorize_url(&mut self) -> (AuthrozationRequest, CsrfToken) {
        let csrf_token = CsrfToken::new_random();

        let auth_request = AuthrozationRequest::new(
            self.get_auth_url(),
            self.client_id.clone(),
            self.redirect_url.clone(),
            ResponseType::Code,
            csrf_token.clone(),
        );

        (auth_request, csrf_token)
    }

    pub async fn exchange_code_for_token(
        &self,
        code: AuthorizationCode,
    ) -> Result<OauthResponse<Token>> {
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

    pub async fn exchange_code(
        &mut self,
        code_state: CodeState,
        csrf_token: CsrfToken,
    ) -> Result<OauthResponse<Token>> {
        match code_state.state {
            ServerStatus::Timeout => Err(Error::TimeoutError("".to_string())),
            ServerStatus::Shutdown => Err(Error::GraceFulShutdown),
            ServerStatus::Recive => {
                let received_csrf = code_state.csrf_token.ok_or(Error::ResponseCsrfTokenError)?;

                if received_csrf.secret() != csrf_token.secret() {
                    return Err(Error::CsrfTokenPartialEqError);
                }

                let code = code_state.code.ok_or(Error::MissingAuthorizationCode)?;
                self.exchange_code_for_token(code).await
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
        access_token: AccessToken,
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

pub struct OauthResponse<RT>
where
    RT: DeserializeOwned,
{
    pub status_code: StatusCode,
    pub body: String,
    _phantom: PhantomData<RT>,
}

impl<RT> fmt::Debug for OauthResponse<RT>
where
    RT: DeserializeOwned,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OauthResponse")
            .field("status_code", &self.status_code)
            .field("body", &self.body)
            .finish()
    }
}

impl<RT> OauthResponse<RT>
where
    RT: DeserializeOwned,
{
    pub fn new(status_code: StatusCode, body: String) -> Self {
        OauthResponse {
            status_code,
            body,
            _phantom: PhantomData,
        }
    }
    pub fn json(self) -> crate::Result<RT> {
        match self.status_code {
            StatusCode::OK => {
                let token: RT = serde_json::from_str(&self.body).unwrap();
                Ok(token)
            }
            _ => {
                let token: ErrorResponse = serde_json::from_str(&self.body).unwrap();
                Err(Error::ResponseError(token))
            }
        }
    }
}
