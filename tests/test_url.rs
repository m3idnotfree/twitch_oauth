#[cfg(test)]
#[test]
fn user_token() {
    use asknothingx2_util::api::{APIRequest, Method};
    use twitch_oauth_token::{types::Scope, TwitchOauth};
    use url::Url;

    let oauth = TwitchOauth::new(
        "cb7b5eba670c41fa757410b811601b",
        "f40fbf26d4e2c20de5772e4408589c",
        None,
    )
    .unwrap()
    .with_url(None);

    let mut token = oauth.user_token("29405430");
    token.scopes_mut().extend([Scope::UserReadEmail]);

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

    assert_eq!(expected_url.to_string(), token.url().to_string());
    assert_eq!(Method::POST, token.method());
}

#[cfg(test)]
#[test]
fn app_token() {
    use asknothingx2_util::api::{APIRequest, Method};
    use twitch_oauth_token::{types::Scope, TwitchOauth};
    use url::Url;

    let oauth = TwitchOauth::new(
        "cb7b5eba670c41fa757410b811601b",
        "f40fbf26d4e2c20de5772e4408589c",
        None,
    )
    .unwrap()
    .with_url(None);

    let mut token = oauth.app_token();
    token.scopes_mut().extend([Scope::UserReadEmail]);

    let expected_scopes = [String::from(Scope::UserReadEmail)].join(" ");

    let expected_params = vec![
        ("client_id", "cb7b5eba670c41fa757410b811601b"),
        ("client_secret", "f40fbf26d4e2c20de5772e4408589c"),
        ("grant_type", "client_credentials"),
    ];

    let mut expected_url = Url::parse("http://localhost:8080/auth/authorize").unwrap();
    expected_url
        .query_pairs_mut()
        .extend_pairs(expected_params)
        .extend_pairs([("scope", expected_scopes)]);

    assert_eq!(expected_url.to_string(), token.url().to_string());
    assert_eq!(Method::POST, token.method());
}
