#![cfg(all(feature = "test", feature = "oneshot-server"))]

use twitch_oauth_token::test_oauth::{OauthTestExt, TestEnv};

#[test]
fn user_token() {
    use asknothingx2_util::api::{request::IntoRequestParts, Method};
    use twitch_oauth_token::{types::Scope, TwitchOauth};
    use url::Url;

    let oauth = TwitchOauth::new(
        "cb7b5eba670c41fa757410b811601b",
        "f40fbf26d4e2c20de5772e4408589c",
    )
    .with_test_env(TestEnv::default());

    let mut token = oauth.create_user_token("29405430");
    token.scopes_mut().push(Scope::UserReadEmail);

    let expected_scopes = [String::from(Scope::UserReadEmail)].join(" ");

    let expected_params = vec![
        ("client_id", "cb7b5eba670c41fa757410b811601b"),
        ("client_secret", "f40fbf26d4e2c20de5772e4408589c"),
        ("grant_type", "user_token"),
        ("user_id", "29405430"),
    ];

    let mut expected_url = Url::parse("http://localhost:8080/auth/authorize").unwrap();
    expected_url
        .query_pairs_mut()
        .extend_pairs(expected_params)
        .extend_pairs([("scope", expected_scopes)]);

    let token = token.into_request_parts();

    assert_eq!(expected_url.to_string(), token.url.to_string());
    assert_eq!(Method::POST, token.method);
}

#[cfg(feature = "test")]
#[test]
fn app_token() {
    use asknothingx2_util::api::{request::IntoRequestParts, Method};
    use twitch_oauth_token::{types::Scope, TwitchOauth};
    use url::Url;

    let oauth = TwitchOauth::new(
        "cb7b5eba670c41fa757410b811601b",
        "f40fbf26d4e2c20de5772e4408589c",
    )
    .with_test_env(TestEnv::default());

    let mut token = oauth.create_app_token();
    token.scopes_mut().push(Scope::UserReadEmail);

    let expected_scopes = [String::from(Scope::UserReadEmail)].join(" ");

    let expected_params = vec![
        ("client_id", "cb7b5eba670c41fa757410b811601b"),
        ("client_secret", "f40fbf26d4e2c20de5772e4408589c"),
        ("grant_type", "client_credentials"),
    ];

    let mut expected_url = Url::parse("http://localhost:8080/auth/token").unwrap();
    expected_url
        .query_pairs_mut()
        .extend_pairs(expected_params)
        .extend_pairs([("scope", expected_scopes)]);

    let token = token.into_request_parts();

    assert_eq!(expected_url.to_string(), token.url.to_string());
    assert_eq!(Method::POST, token.method);
}

// twitch mock-api start
#[cfg(all(feature = "test", feature = "oneshot-server"))]
#[tokio::test]
async fn twitch_mock_server() {
    use asknothingx2_util::api::request::IntoRequestParts;
    use dotenv::dotenv;
    use twitch_oauth_token::{
        test_oauth::{get_users_info, OauthTestExt, TestEnv},
        types::{Scope, Token},
        TwitchOauth,
    };

    dotenv().ok();

    let user_id = std::env::var("USER_ID").expect("USER_ID environment variable is not set");
    let users_info = get_users_info(None)
        .await
        .expect("Failed to connect to Twitch mock server");
    let user = users_info
        .data
        .first()
        .expect("Mock server returned empty user data");

    let test_oauth = TwitchOauth::from_credentials(user.ID.clone(), user.Secret.clone())
        .expect("Failed to initialize TwitchOAuth with mock credentials")
        .with_test_env(TestEnv::default());

    // Getting a user access token
    let mut test_user = test_oauth.create_user_token(&user_id);
    test_user.scopes_mut().push(Scope::ChannelReadPolls);

    let test_user = test_user.into_request_parts();

    let user_token = test_user
        .send()
        .await
        .expect("Failed to request user access token from mock server");
    let user_token = user_token
        .json::<Token>()
        .await
        .expect("Failed to deserialize user access token response");
    assert_eq!(vec![Scope::ChannelReadPolls], user_token.scope);

    // Getting an app access token
    let mut app_token = test_oauth.create_app_token();
    app_token.scopes_mut().clear();

    let app_token = app_token.into_request_parts();

    let app_token = app_token
        .send()
        .await
        .expect("Failed to request app access token from mock server");
    let app_token = app_token
        .json::<Token>()
        .await
        .expect("Failed to deserialize app access token response");
    assert!(app_token.scope.is_empty());
}
