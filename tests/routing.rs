use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use whop_sdk::prelude::*;
use whop_sdk::{Environment, SandboxUrls};

#[test]
fn production_sandbox_and_explicit_urls_resolve_independently() {
    let production = ClientConfig::default();
    assert_eq!(
        production.service_url(|env| env.api_url()),
        "https://api.whop.com/api/v1"
    );
    assert_eq!(
        production.service_url(|env| env.vault_url()),
        "https://vault-api.whop.com/api/v1"
    );
    let sandbox = ClientConfig {
        environment: Some(Environment::sandbox()),
        ..Default::default()
    };
    assert_eq!(
        sandbox.service_url(|env| env.api_url()),
        "https://sandbox-api.whop.com/api/v1"
    );
    assert_eq!(
        sandbox.service_url(|env| env.vault_url()),
        "https://sandbox-vault-api.whop.com/api/v1"
    );
    let explicit = ClientConfig {
        base_url: "http://localhost:1234/custom".into(),
        environment: Some(Environment::sandbox()),
        ..Default::default()
    };
    assert_eq!(
        explicit.service_url(|env| env.api_url()),
        "http://localhost:1234/custom"
    );
    assert_eq!(
        explicit.service_url(|env| env.vault_url()),
        "http://localhost:1234/custom"
    );
}

#[tokio::test]
async fn generated_requests_use_the_selected_environment_and_explicit_override() {
    for explicit in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let origin = format!("http://{address}");
        let transport = reqwest::Client::builder()
            .no_proxy()
            .resolve("api.whop.com", address)
            .resolve("sandbox-api.whop.com", address)
            .resolve("vault-api.whop.com", address)
            .resolve("sandbox-vault-api.whop.com", address)
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let mut config = ClientConfig {
            environment: Some(Environment::Sandbox(SandboxUrls {
                api: format!("{origin}/sandbox"),
                vault: format!("{origin}/vault"),
            })),
            token: Some("audit-test-token".into()),
            max_retries: 0,
            reqwest_client: Some(transport),
            ..Default::default()
        };
        if explicit {
            config.base_url = format!("{origin}/override");
        }
        let client = Whop::new(config).unwrap();
        let request = tokio::spawn(async move {
            let _ = client
                .users
                .retrieve("me", &UsersRetrieveQueryRequest::default(), None)
                .await;
        });
        let (mut stream, _) = tokio::time::timeout(Duration::from_secs(5), listener.accept())
            .await
            .unwrap()
            .unwrap();
        let mut bytes = Vec::new();
        while !bytes.windows(4).any(|window| window == b"\r\n\r\n") && bytes.len() < 16384 {
            let mut chunk = [0; 4096];
            let count = tokio::time::timeout(Duration::from_secs(5), stream.read(&mut chunk))
                .await
                .unwrap()
                .unwrap();
            if count == 0 {
                break;
            }
            bytes.extend_from_slice(&chunk[..count]);
        }
        let headers = String::from_utf8_lossy(&bytes);
        let prefix = if explicit { "override" } else { "sandbox" };
        assert!(
            headers.starts_with(&format!("GET /{prefix}/users/me ")),
            "{headers}"
        );
        assert!(headers.to_lowercase().contains("api-version-date: "));
        stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}").await.unwrap();
        request.await.unwrap();
    }
}
