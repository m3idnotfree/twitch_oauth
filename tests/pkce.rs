use twitch_oauth_token::pkce::Pkce;

#[test]
fn pkce_base() {
    let (pkce_challenge, code_virify) = Pkce::new_sha256().unwrap();
    code_virify(pkce_challenge).unwrap();
}
