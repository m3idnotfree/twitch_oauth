use base64::{engine::general_purpose::URL_SAFE, Engine};
use rand::{distributions::Alphanumeric, Rng};
use ring::{
    digest,
    error::Unspecified,
    hmac::{self, Key},
};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    #[error("Failed decode base64: {0}")]
    DecodeBase64Error(#[from] base64::DecodeError),
    #[error("Failed verify: {0}")]
    VerifyHmacError(#[from] ring::error::Unspecified),
}

#[derive(Debug)]
pub struct Pkce {}

impl Pkce {
    pub fn new_sha256() -> Result<(String, impl Fn(String) -> Result<(), Error>), Unspecified> {
        let msg: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(24)
            .map(char::from)
            .collect();

        let rng = ring::rand::SystemRandom::new();
        let key_value: [u8; digest::SHA256_OUTPUT_LEN] = ring::rand::generate(&rng)?.expose();

        let key = hmac::Key::new(hmac::HMAC_SHA256, key_value.as_ref());
        let tag = hmac::sign(&key, msg.as_bytes());

        let pkce_challenge = URL_SAFE.encode(tag.as_ref());
        Ok((pkce_challenge, pkce_verify(msg, key)))
    }
}

fn pkce_verify(msg: String, key: Key) -> impl Fn(String) -> Result<(), Error> {
    move |tag| {
        let url_decode = URL_SAFE.decode(tag)?;
        hmac::verify(&key, msg.as_bytes(), url_decode.as_ref())?;
        Ok(())
    }
}
