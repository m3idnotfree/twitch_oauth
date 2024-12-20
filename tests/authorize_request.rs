use asknothingx2_util::oauth::{AuthUrl, ClientId, CsrfToken, RedirectUrl};
use twitch_oauth_token::{request::AuthrozationRequest, scopes::Scopes, types::ResponseType};

#[test]
fn request() {
    let csrf_token = CsrfToken::new_random();
    let mut request = AuthrozationRequest::new(
        AuthUrl::new("https://id.twitch.tv/oauth2/authorize".to_string()).unwrap(),
        ClientId::new("test_id".to_string()),
        RedirectUrl::new("http://localhost:3000".to_string()).unwrap(),
        ResponseType::Token,
        csrf_token.clone(),
    );

    request
        .scopes_mut()
        .push(Scopes::ChatRead)
        .extend([
            Scopes::ChannelManageSchedule,
            Scopes::ModeratorManageAutomod,
        ])
        .push(Scopes::UserBot)
        .push(Scopes::ChatRead);

    let expected_url = request.url().to_string();
    assert!(expected_url.contains("chat%3Aread"));
    assert!(expected_url.contains("channel%3Amanage%3Aschedule"));
    assert!(expected_url.contains("manage%3Aautomod"));
    assert!(expected_url.contains("user%3Abot"));
}
