//! Guards the hand-written OIDC helper. src/lib.rs is generated, and `pub mod oidc;` in it is carried
//! across regenerations by Fern Replay; if a regeneration ever drops it, this file stops compiling.

use std::sync::{Arc, Mutex};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use whop_sdk::oidc::{self, IdentityToken, OidcError};
use whop_sdk::{ApiClientBuilder, UsersRetrieveQueryRequest};

async fn serve(responses: Vec<&'static str>) -> (String, Arc<Mutex<Vec<String>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let heads = Arc::new(Mutex::new(Vec::new()));
    let sink = heads.clone();
    tokio::spawn(async move {
        for body in responses {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buffer = Vec::new();
            let mut chunk = [0u8; 4096];
            let header_end = loop {
                let read = socket.read(&mut chunk).await.unwrap();
                buffer.extend_from_slice(&chunk[..read]);
                if let Some(position) = buffer.windows(4).position(|window| window == b"\r\n\r\n") {
                    break position + 4;
                }
            };
            let head = String::from_utf8_lossy(&buffer[..header_end]).to_string();
            let length = head
                .lines()
                .find_map(|line| {
                    line.to_ascii_lowercase()
                        .strip_prefix("content-length: ")
                        .map(|value| value.trim().parse::<usize>().unwrap_or(0))
                })
                .unwrap_or(0);
            while buffer.len() < header_end + length {
                let read = socket.read(&mut chunk).await.unwrap();
                buffer.extend_from_slice(&chunk[..read]);
            }
            sink.lock().unwrap().push(head);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        }
    });
    (address, heads)
}

#[tokio::test]
async fn the_client_built_with_the_exchanged_token_sends_it() {
    let (address, heads) = serve(vec![
        r#"{"access_token":"whop-token-1","expires_in":3600}"#,
        r#"{"id":"user_xxxxxxxxxxxxx"}"#,
    ])
    .await;

    let token = oidc::Exchange::new(
        "aurl_xxxxxxxxxxxxx",
        IdentityToken::Value("id-token".to_string()),
    )
    .base_url(address.clone())
    .access_token()
    .await
    .unwrap();
    let whop = ApiClientBuilder::new(address)
        .token(token)
        .max_retries(0)
        .build()
        .unwrap();
    let _ = whop
        .users
        .retrieve(
            "user_xxxxxxxxxxxxx",
            &UsersRetrieveQueryRequest::default(),
            None,
        )
        .await;

    let api_call = heads.lock().unwrap()[1].clone();
    assert!(
        api_call.starts_with("GET /users/user_xxxxxxxxxxxxx"),
        "{api_call}"
    );
    assert!(
        api_call
            .to_ascii_lowercase()
            .contains("authorization: bearer whop-token-1"),
        "{api_call}"
    );
}

#[tokio::test]
async fn github_actions_requests_the_whop_audience_and_explains_a_missing_permission() {
    std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_URL");
    std::env::remove_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN");
    let missing = oidc::access_token("aurl_xxxxxxxxxxxxx", IdentityToken::github_actions())
        .await
        .unwrap_err();
    assert!(
        matches!(missing, OidcError::Config(ref message) if message.contains("id-token: write")),
        "{missing:?}"
    );

    let (address, heads) = serve(vec![
        r#"{"value":"github-id-token"}"#,
        r#"{"access_token":"whop-token-1","expires_in":3600}"#,
    ])
    .await;
    std::env::set_var(
        "ACTIONS_ID_TOKEN_REQUEST_URL",
        format!("{address}/idtoken?api-version=2.0"),
    );
    std::env::set_var("ACTIONS_ID_TOKEN_REQUEST_TOKEN", "request-token");

    let token = oidc::Exchange::new("aurl_xxxxxxxxxxxxx", IdentityToken::github_actions())
        .base_url(address)
        .access_token()
        .await
        .unwrap();

    assert_eq!(token, "whop-token-1");
    let github_request = heads.lock().unwrap()[0].clone();
    assert!(
        github_request
            .starts_with("GET /idtoken?api-version=2.0&audience=https%3A%2F%2Fapi.whop.com"),
        "{github_request}"
    );
    assert!(
        github_request
            .to_ascii_lowercase()
            .contains("authorization: bearer request-token"),
        "{github_request}"
    );
    assert!(heads.lock().unwrap()[1].starts_with("POST /access_tokens/exchange"));
}
