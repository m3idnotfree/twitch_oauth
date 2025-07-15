use twitch_oauth_token::{csrf, types::Scope, AuthUrl, AuthrozationRequest, ClientId, RedirectUrl};

#[test]
fn request() {
    let auth_url = AuthUrl::new("https://id.twitch.tv/oauth2/authorize".into()).unwrap();
    let client_id = ClientId::new("test_id".into());
    let redirect_url = RedirectUrl::new("http://localhost:3000".into()).unwrap();
    let mut request = AuthrozationRequest::new(
        &auth_url,
        &client_id,
        Some(&redirect_url),
        csrf::generate(&csrf::generate_secret_key(), None),
    );

    request
        .scopes_mut()
        .push(Scope::ChatRead)
        .extend([Scope::ChannelManageSchedule, Scope::ModeratorManageAutomod])
        .push(Scope::UserBot)
        .push(Scope::ChatRead);

    let expected_url = request.url().unwrap().to_string();
    assert!(expected_url.contains("chat%3Aread"));
    assert!(expected_url.contains("channel%3Amanage%3Aschedule"));
    assert!(expected_url.contains("manage%3Aautomod"));
    assert!(expected_url.contains("user%3Abot"));
    assert!(!expected_url.contains("force_verify"));
}

#[test]
fn request_not_include_scopes() {
    let auth_url = AuthUrl::new("https://i.twitch.tv/oauth2/authorize".into()).unwrap();
    let client_id = ClientId::new("test_id".into());
    let redirect_url = RedirectUrl::new("http://localhost:3000".into()).unwrap();
    let request = AuthrozationRequest::new(
        &auth_url,
        &client_id,
        Some(&redirect_url),
        csrf::generate(&csrf::generate_secret_key(), None),
    );

    let expected_url = request.url().unwrap().to_string();
    assert!(!expected_url.contains("scope"));
    assert!(!expected_url.contains("force_verify"));
}

#[test]
fn request_force_verify() {
    let auth_url = AuthUrl::new("https://id.twitch.tv/oauth2/authorize".into()).unwrap();
    let client_id = ClientId::new("test_id".into());
    let redirect_url = RedirectUrl::new("http://localhost:3000".into()).unwrap();

    let mut request = AuthrozationRequest::new(
        &auth_url,
        &client_id,
        Some(&redirect_url),
        csrf::generate(&csrf::generate_secret_key(), None),
    );
    request.set_force_verify(true);

    let expected_url = request.url().unwrap().to_string();
    assert!(!expected_url.contains("scope"));
    assert!(expected_url.contains("force_verify=true"));
}
