use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{DateTime, TimeZone, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsrfConfig {
    pub clock_skew: Option<u64>,
    pub max_age: u64,
}

impl Default for CsrfConfig {
    fn default() -> Self {
        Self {
            clock_skew: None,
            // 30 minues
            max_age: 1800,
        }
    }
}

impl CsrfConfig {
    pub fn new(clock_skew: u64, max_age: u64) -> Self {
        Self {
            clock_skew: if clock_skew > 0 {
                Some(clock_skew)
            } else {
                None
            },
            max_age,
        }
    }

    pub fn with_clock_skew(mut self, clock_skew: u64) -> Self {
        self.clock_skew = if clock_skew > 0 {
            Some(clock_skew)
        } else {
            None
        };
        self
    }

    pub fn with_max_age(mut self, max_age: u64) -> Self {
        self.max_age = max_age;
        self
    }
}

pub fn generate(secret_key: &[u8; 32], user_id: Option<&str>) -> String {
    let timestamp = current_timestamp();
    let payload = format!("{timestamp}:{}", user_id.unwrap_or(""));

    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret_key).expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    let signature = mac.finalize().into_bytes();

    let token_data = format!("{timestamp}:{}", hex::encode(signature));
    URL_SAFE_NO_PAD.encode(token_data.as_bytes())
}

pub fn validate(
    secret_key: &[u8; 32],
    token: &str,
    user_id: Option<&str>,
    max_age_seconds: u64,
) -> bool {
    validate_with_time_and_config(
        secret_key,
        token,
        user_id,
        current_timestamp(),
        CsrfConfig::default().with_max_age(max_age_seconds),
    )
}

pub fn validate_with_config(
    secret_key: &[u8; 32],
    token: &str,
    user_id: Option<&str>,
    config: CsrfConfig,
) -> bool {
    validate_with_time_and_config(secret_key, token, user_id, current_timestamp(), config)
}

pub fn validate_with_time_and_config(
    secret_key: &[u8; 32],
    token: &str,
    user_id: Option<&str>,
    validation_time: i64,
    config: CsrfConfig,
) -> bool {
    let decoded = match URL_SAFE_NO_PAD.decode(token) {
        Ok(data) => match String::from_utf8(data) {
            Ok(s) => s,
            Err(_) => return false,
        },
        Err(_) => return false,
    };

    let parts: Vec<&str> = decoded.split(':').collect();
    if parts.len() != 2 {
        return false;
    }

    let timestamp: i64 = match parts[0].parse() {
        Ok(ts) => ts,
        Err(_) => return false,
    };

    let provided_signature = match hex::decode(parts[1]) {
        Ok(sig) => sig,
        Err(_) => return false,
    };

    if timestamp < 0 {
        return false;
    }

    let age = validation_time - timestamp;
    let tolerance = config.clock_skew.unwrap_or(0) as i64; // Cast to i64

    let max_age = config.max_age as i64;
    let effective_max_age = max_age + tolerance;
    let min_age = -tolerance;

    if age > effective_max_age || age < min_age {
        return false;
    }

    let payload = format!("{}:{}", timestamp, user_id.unwrap_or(""));
    let mut mac =
        Hmac::<Sha256>::new_from_slice(secret_key).expect("HMAC can accept keys of any size");
    mac.update(payload.as_bytes());

    mac.verify_slice(&provided_signature).is_ok()
}

pub fn is_expired(token: &str, max_age_seconds: u64) -> Option<bool> {
    let timestamp = extract_timestamp(token)?;

    if timestamp < 0 {
        return Some(true);
    }

    let now = current_timestamp();
    let age = now - timestamp;
    if age < 0 {
        return Some(true);
    }
    Some(age > max_age_seconds as i64)
}

pub fn extract_datetime(token: &str) -> Option<DateTime<Utc>> {
    Utc.timestamp_opt(extract_timestamp(token)?, 0).single()
}

pub fn token_age(token: &str) -> Option<i64> {
    let timestamp = extract_timestamp(token)?;
    let now = current_timestamp();
    Some(now - timestamp)
}

pub fn extract_timestamp(token: &str) -> Option<i64> {
    let decoded = URL_SAFE_NO_PAD.decode(token).ok()?;
    let decoded_str = String::from_utf8(decoded).ok()?;
    let timestamp_str = decoded_str.split(':').next()?;
    timestamp_str.parse().ok()
}

pub fn generate_secret_key() -> [u8; 32] {
    rand::random()
}

pub fn current_timestamp() -> i64 {
    Utc::now().timestamp()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[tokio::test]
    async fn basic() {
        let secret_key = generate_secret_key();
        let token = generate(&secret_key, Some("user123"));

        assert!(validate(&secret_key, &token, Some("user123"), 3600));
        assert!(!validate(&secret_key, &token, Some("user456"), 3600));

        tokio::time::sleep(Duration::from_secs(1)).await;
        assert!(!validate(&secret_key, &token, Some("user123"), 0));
    }

    #[tokio::test]
    async fn with_tolerance() {
        let secret_key = generate_secret_key();
        let token = generate(&secret_key, Some("user123"));

        assert!(validate_with_config(
            &secret_key,
            &token,
            Some("user123"),
            CsrfConfig::new(3, 3600)
        ));
        assert!(!validate_with_config(
            &secret_key,
            &token,
            Some("user456"),
            CsrfConfig::new(3, 3600)
        ));

        tokio::time::sleep(Duration::from_secs(1)).await;
        assert!(validate_with_config(
            &secret_key,
            &token,
            Some("user123"),
            CsrfConfig::new(3, 0)
        ));
    }

    #[test]
    fn extract() {
        let current_time = super::current_timestamp();
        let secret_key = generate_secret_key();
        let token = generate(&secret_key, Some("user123"));

        let extract_timestamp = extract_timestamp(&token);
        assert_eq!(current_time, extract_timestamp.unwrap());
    }
}
