use twitch_oauth_token::{
    test_url::get_users_info,
    types::{PollsScopes, Scope},
    TwitchOauth,
};

// cargo run example twitch_mock --features test
#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let user_id = std::env::var("USER_ID").expect("Failed read env Variable: USER_ID");
    // Does not contain a user_id
    // When first run twitch mock-api generate
    // copy user_id
    // https://dev.twitch.tv/docs/cli/mock-api-command/#getting-an-access-token
    let users_info = get_users_info(None)
        .await
        .expect("Failed to connect to Twitch mock server");
    let user = users_info
        .data
        .first()
        .expect("Mock server returned empty user data");

    let test_oauth = TwitchOauth::from_credentials(user.ID.clone(), user.Secret.clone(), None)
        .expect("Failed to parse redirect URL in TwitchOauth initialization")
        .with_url(None);

    // Getting a user access token
    let mut test_user = test_oauth.user_token(user_id);
    test_user.scopes_mut().with_polls_read();

    let user_token = test_user
        .request_access_token()
        .await
        .expect("Failed to request user access token from mock server");
    let user_token = user_token
        .parse_token()
        .expect("Failed to deserialize user access token response");
    assert_eq!(vec![Scope::ChannelReadPolls], user_token.scope);

    // Getting an app access token
    let mut app_token = test_oauth.app_token();
    app_token.scopes_mut().clear();

    let app_token = app_token
        .request_access_token()
        .await
        .expect("Failed to request app access token from mock server");
    let app_token = app_token
        .parse_token()
        .expect("Failed to deserialize app access token response");
    assert!(app_token.scope.is_empty());
}
