use asknothingx2_util::{
    api::{api_request, StatusCode},
    oauth::{ClientId, ClientSecret, RefreshToken, TokenUrl},
};
use twitch_oauth_token::{request::RefreshRequest, types::GrantType};

mod server;
#[tokio::test]
async fn refresh_token() {
    let mock_uri = server::refresh().await;

    let a = api_request(RefreshRequest {
        client_id: &ClientId::new("hof5gwx0su6owfnys0yan9c87zr6t".to_string()),
        client_secret: &ClientSecret::new("".to_string()),
        grant_type: GrantType::RefreshToken,
        token_url: &TokenUrl::new(format!("{}/refresh", mock_uri)).unwrap(),
        refresh_token: &RefreshToken::new(
            "gdw3k62zpqi0kw01escg7zgbdhtxi6hm0155tiwcztxczkx17".to_string(),
        ),
    })
    .await
    .unwrap();

    assert_eq!(StatusCode::OK, a.status());
}
