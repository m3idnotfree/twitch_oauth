use twitch_oauth_token::TwitchOauth;

#[test]
fn twitch_base_default() {
    let base = TwitchOauth::default()
        .set_client_id("client_id")
        .set_client_secret("client_secret");

    assert_eq!(60928, base.port);

    // let auth = base.authorize_url().url();

    // let state = auth
    //     .query_pairs()
    //     .find(|(key, _)| key == "state")
    //     .map(|(_, state)| CsrfToken::new(state.into_owned()))
    //     .unwrap();

    // exchange_code
    // exchange_code_with_statuscode
    // exchange_refresh_token
    // validate_token
    // revoke_token
}

#[test]
fn twitch_base_set_redirct() {
    let base = TwitchOauth::default()
        .set_client_id("client_id")
        .set_client_secret("client_secret")
        .set_redirect_uri("http://localhost:3000");
    assert!(base.is_ok());

    let base = base.unwrap();
    assert_eq!(3000, base.port);

    // let auth = base.authorize_url().url();

    // let state = auth
    //     .query_pairs()
    //     .find(|(key, _)| key == "state")
    //     .map(|(_, state)| CsrfToken::new(state.into_owned()))
    //     .unwrap();

    // assert!(base.csrf_cmp(state));
}

#[test]
fn err_redir_url() {
    let base = TwitchOauth::default().set_redirect_uri("localhost:3000");
    assert!(base.is_err());
    // assert_eq!(
    //     "RedirectUrlError(must set schema http or https)",
    //     format!("{:?}", base.unwrap_err())
    // );
}
