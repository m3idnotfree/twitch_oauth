use twitch_oauth_token::{
    oauth_types::{AuthorizationCode, RedirectUrl},
    TwitchOauth,
};

#[tokio::test]
async fn csrf_validation_failure() {
    let oauth = TwitchOauth::new("client_id", "client_secret")
        .set_redirect_uri(RedirectUrl::new("http://localhost:3000".to_string()).unwrap());

    let result = oauth
        .user_access_token(
            AuthorizationCode::new("code".to_string()),
            "state".to_string(),
        )
        .await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.is_oauth_error());
    assert_eq!(
        error.message(),
        Some("CSRF token validation failed - possible security issue")
    );
}

#[cfg(feature = "oneshot-server")]
mod oneshot_tests {
    use reqwest::Client;
    use std::time::Duration;
    use tokio::{net::TcpListener, time::sleep};
    use twitch_oauth_token::oneshot_server::{oneshot_server, ServerError};

    #[tokio::test]
    async fn test_oneshot_server_timeout() {
        let result = oneshot_server("127.0.0.1:0", Duration::from_millis(100)).await;
        assert!(matches!(result, Err(ServerError::Timeout)));
    }

    #[tokio::test]
    async fn oauth_callback() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let port = addr.port();
        drop(listener);

        let bind_addr = format!("127.0.0.1:{port}");

        let server_future = oneshot_server(&bind_addr, Duration::from_secs(5));

        // Spawn HTTP client to make the OAuth callback request
        tokio::spawn(async move {
            sleep(Duration::from_millis(100)).await;

            let client = Client::new();
            let callback_url = format!(
                "http://127.0.0.1:{port}/?code=test_code&state=test_state&scope=channel:read"
            );

            if let Ok(response) = client.get(&callback_url).send().await {
                // Verify the server responds with success message
                if let Ok(json) = response.json::<serde_json::Value>().await {
                    assert_eq!(
                        json["message"],
                        "Authorization successful! You can close this window."
                    );
                }
            }
        });

        let result = server_future.await;
        match result {
            Ok(callback) => {
                assert_eq!(callback.code.secret(), "test_code");
                assert_eq!(callback.state, "test_state");
                assert_eq!(callback.scope, "channel:read");
            }
            Err(e) => panic!("Expected successful callback, got: {e:?}"),
        }
    }

    #[tokio::test]
    async fn invalid_request() {
        use tokio::io::AsyncWriteExt;
        use tokio::net::TcpStream;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let port = addr.port();
        drop(listener);

        let bind_addr = format!("127.0.0.1:{port}");
        let connect_addr = bind_addr.clone();

        let server_future = oneshot_server(&bind_addr, Duration::from_secs(5));

        tokio::spawn(async move {
            sleep(Duration::from_millis(100)).await;
            if let Ok(mut stream) = TcpStream::connect(&connect_addr).await {
                let _ = stream.write_all(b"INVALID REQUEST\r\n").await;
            }
        });

        let result = server_future.await;
        assert!(result.is_err());
    }
}
