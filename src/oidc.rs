//! Authenticate with your cloud provider's OIDC identity token instead of an API key.
//!
//! [`access_token`] exchanges an identity token for a short-lived Whop access token that assumes a
//! role. The client sets its bearer token once, when it is built, so build it with the token:
//!
//! ```no_run
//! # async fn run() -> Result<(), Box<dyn std::error::Error>> {
//! use whop_sdk::oidc::{self, IdentityToken};
//! use whop_sdk::ApiClientBuilder;
//!
//! let token = oidc::access_token("aurl_xxxxxxxxxxxxx", IdentityToken::github_actions()).await?;
//! let whop = ApiClientBuilder::default().token(token).build()?;
//! # Ok(())
//! # }
//! ```
//!
//! [`IdentityToken::github_actions`] and [`IdentityToken::google_cloud`] request
//! [`DEFAULT_AUDIENCE`]. For a trust with another audience, use
//! [`IdentityToken::github_actions_with_audience`] or
//! [`IdentityToken::google_cloud_with_audience`].
//!
//! The token lasts an hour by default and up to three with [`Exchange::expires_at`]. A long-running
//! process builds a new client with a fresh token before the old one expires. The module depends on
//! no generated code, so it survives the client being regenerated.

use std::time::Duration;

use chrono::{DateTime, SecondsFormat, Utc};
use serde::Deserialize;

/// The audience [`IdentityToken::github_actions`] and [`IdentityToken::google_cloud`] request, and
/// the one to set on your trust.
pub const DEFAULT_AUDIENCE: &str = "https://api.whop.com";

const DEFAULT_BASE_URL: &str = "https://api.whop.com/api/v1";
const GRANT_TYPE: &str = "urn:ietf:params:oauth:grant-type:token-exchange";
const ID_TOKEN_TYPE: &str = "urn:ietf:params:oauth:token-type:id_token";
const HTTP_TIMEOUT: Duration = Duration::from_secs(10);
const GOOGLE_METADATA_IDENTITY_URL: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/identity";

/// Where the identity token comes from.
#[derive(Debug, Clone)]
pub enum IdentityToken {
    /// A token you already hold.
    Value(String),
    /// The job's token from GitHub Actions. The job needs `permissions: id-token: write`.
    GitHubActions {
        /// The audience to request. It must equal the audience on your trust.
        audience: String,
    },
    /// The token of the service account the workload runs as, from the Google Cloud metadata
    /// server, on Cloud Run, Cloud Functions, Compute Engine, or GKE with Workload Identity.
    GoogleCloud {
        /// The audience to request. It must equal the audience on your trust.
        audience: String,
    },
}

impl IdentityToken {
    /// The job's token from GitHub Actions, for [`DEFAULT_AUDIENCE`].
    pub fn github_actions() -> Self {
        Self::github_actions_with_audience(DEFAULT_AUDIENCE)
    }

    /// The job's token from GitHub Actions, for `audience`. It must equal the audience on your
    /// trust.
    pub fn github_actions_with_audience(audience: impl Into<String>) -> Self {
        Self::GitHubActions {
            audience: audience.into(),
        }
    }

    /// The service account's token from the Google Cloud metadata server, for [`DEFAULT_AUDIENCE`].
    pub fn google_cloud() -> Self {
        Self::google_cloud_with_audience(DEFAULT_AUDIENCE)
    }

    /// The service account's token from the Google Cloud metadata server, for `audience`. It must
    /// equal the audience on your trust.
    pub fn google_cloud_with_audience(audience: impl Into<String>) -> Self {
        Self::GoogleCloud {
            audience: audience.into(),
        }
    }

    async fn resolve(&self, http: &reqwest::Client) -> Result<String, OidcError> {
        match self {
            Self::Value(token) => Ok(token.clone()),
            Self::GitHubActions { audience } => github_actions_token(http, audience).await,
            Self::GoogleCloud { audience } => {
                google_cloud_token(http, GOOGLE_METADATA_IDENTITY_URL, audience).await
            }
        }
    }
}

/// A failed exchange or identity token request.
#[derive(Debug, thiserror::Error)]
pub enum OidcError {
    /// The API or the provider refused the request. `code` is the API's error code when it sent one.
    #[error("{message}")]
    Refused {
        status: u16,
        code: Option<String>,
        message: String,
    },
    /// The request could not be completed.
    #[error(transparent)]
    Http(#[from] reqwest::Error),
    /// The exchange is missing a role or the environment is missing a provider's token.
    #[error("{0}")]
    Config(String),
}

/// An exchange of an identity token for a Whop access token.
#[derive(Debug, Clone)]
pub struct Exchange {
    role_id: String,
    identity_token: IdentityToken,
    base_url: String,
    expires_at: Option<DateTime<Utc>>,
}

impl Exchange {
    /// An exchange for an access token that assumes `role_id`, prefixed `aurl_`. A trust linked to
    /// the role must accept the identity token.
    pub fn new(role_id: impl Into<String>, identity_token: IdentityToken) -> Self {
        Self {
            role_id: role_id.into(),
            identity_token,
            base_url: DEFAULT_BASE_URL.to_string(),
            expires_at: None,
        }
    }

    /// The API's base URL, for example `https://sandbox-api.whop.com/api/v1`. Defaults to production.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// When the access token should expire, up to three hours from now. Defaults to an hour.
    pub fn expires_at(mut self, expires_at: DateTime<Utc>) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    /// Exchanges the identity token and returns the access token.
    pub async fn access_token(&self) -> Result<String, OidcError> {
        if self.role_id.is_empty() {
            return Err(OidcError::Config(
                "Pass the role to assume as role_id.".to_string(),
            ));
        }

        let http = reqwest::Client::builder().timeout(HTTP_TIMEOUT).build()?;
        let subject_token = self.identity_token.resolve(&http).await?;
        let mut body = serde_json::json!({
            "grant_type": GRANT_TYPE,
            "subject_token_type": ID_TOKEN_TYPE,
            "subject_token": subject_token,
            "role_id": self.role_id,
        });
        if let Some(expires_at) = self.expires_at {
            body["expires_at"] =
                serde_json::Value::String(expires_at.to_rfc3339_opts(SecondsFormat::Secs, true));
        }

        let url = format!(
            "{}/access_tokens/exchange",
            self.base_url.trim_end_matches('/')
        );
        let response = http.post(url).json(&body).send().await?;
        let status = response.status();
        let payload: ExchangeResponse =
            serde_json::from_slice(&response.bytes().await?).unwrap_or_default();

        if !status.is_success() {
            let error = payload.error.unwrap_or_default();
            return Err(OidcError::Refused {
                status: status.as_u16(),
                code: error.code,
                message: error.message.unwrap_or_else(|| {
                    format!("The token exchange failed with {}.", status.as_u16())
                }),
            });
        }
        payload.access_token.ok_or_else(|| OidcError::Refused {
            status: status.as_u16(),
            code: None,
            message: "The token exchange returned no access token.".to_string(),
        })
    }
}

/// Exchanges an identity token for a Whop access token that assumes `role_id`, prefixed `aurl_`.
pub async fn access_token(
    role_id: impl Into<String>,
    identity_token: IdentityToken,
) -> Result<String, OidcError> {
    Exchange::new(role_id, identity_token).access_token().await
}

#[derive(Debug, Default, Deserialize)]
struct ExchangeResponse {
    access_token: Option<String>,
    error: Option<ExchangeError>,
}

#[derive(Debug, Default, Deserialize)]
struct ExchangeError {
    message: Option<String>,
    code: Option<String>,
}

async fn github_actions_token(http: &reqwest::Client, audience: &str) -> Result<String, OidcError> {
    let request_url = std::env::var("ACTIONS_ID_TOKEN_REQUEST_URL").unwrap_or_default();
    let request_token = std::env::var("ACTIONS_ID_TOKEN_REQUEST_TOKEN").unwrap_or_default();
    if request_url.is_empty() || request_token.is_empty() {
        return Err(OidcError::Config(
            "GitHub Actions did not provide an identity token. Give the job permissions: id-token: write.".to_string(),
        ));
    }

    let response = http
        .get(request_url)
        .query(&[("audience", audience)])
        .bearer_auth(request_token)
        .send()
        .await?;
    let status = response.status();
    if !status.is_success() {
        return Err(OidcError::Refused {
            status: status.as_u16(),
            code: None,
            message: format!(
                "GitHub Actions refused the identity token request with {}.",
                status.as_u16()
            ),
        });
    }

    #[derive(Deserialize)]
    struct GitHubToken {
        value: Option<String>,
    }
    let token: GitHubToken =
        serde_json::from_slice(&response.bytes().await?).unwrap_or(GitHubToken { value: None });
    token.value.ok_or_else(|| OidcError::Refused {
        status: status.as_u16(),
        code: None,
        message: "GitHub Actions returned no identity token.".to_string(),
    })
}

async fn google_cloud_token(
    http: &reqwest::Client,
    metadata_url: &str,
    audience: &str,
) -> Result<String, OidcError> {
    let response = http
        .get(metadata_url)
        .query(&[("audience", audience)])
        .header("Metadata-Flavor", "Google")
        .send()
        .await?;
    let status = response.status();
    if !status.is_success() {
        return Err(OidcError::Refused {
            status: status.as_u16(),
            code: None,
            message: format!(
                "The Google Cloud metadata server refused the identity token request with {}.",
                status.as_u16()
            ),
        });
    }
    Ok(response.text().await?.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    #[derive(Debug, Clone)]
    struct Recorded {
        request_line: String,
        headers: Vec<(String, String)>,
        body: String,
    }

    impl Recorded {
        fn header(&self, name: &str) -> Option<&str> {
            self.headers
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(name))
                .map(|(_, value)| value.as_str())
        }
    }

    async fn serve(responses: Vec<(u16, &'static str)>) -> (String, Arc<Mutex<Vec<Recorded>>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = format!("http://{}", listener.local_addr().unwrap());
        let recorded = Arc::new(Mutex::new(Vec::new()));
        let sink = recorded.clone();
        tokio::spawn(async move {
            for (status, body) in responses {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut buffer = Vec::new();
                let mut chunk = [0u8; 4096];
                let header_end = loop {
                    let read = socket.read(&mut chunk).await.unwrap();
                    buffer.extend_from_slice(&chunk[..read]);
                    if let Some(position) =
                        buffer.windows(4).position(|window| window == b"\r\n\r\n")
                    {
                        break position + 4;
                    }
                };
                let head = String::from_utf8_lossy(&buffer[..header_end]).to_string();
                let mut lines = head.lines();
                let request_line = lines.next().unwrap_or_default().to_string();
                let headers: Vec<(String, String)> = lines
                    .filter_map(|line| {
                        line.split_once(": ")
                            .map(|(key, value)| (key.to_string(), value.to_string()))
                    })
                    .collect();
                let length = headers
                    .iter()
                    .find(|(key, _)| key.eq_ignore_ascii_case("content-length"))
                    .and_then(|(_, value)| value.parse::<usize>().ok())
                    .unwrap_or(0);
                while buffer.len() < header_end + length {
                    let read = socket.read(&mut chunk).await.unwrap();
                    buffer.extend_from_slice(&chunk[..read]);
                }
                let body_text =
                    String::from_utf8_lossy(&buffer[header_end..header_end + length]).to_string();
                sink.lock().unwrap().push(Recorded {
                    request_line,
                    headers,
                    body: body_text,
                });
                let response = format!(
                    "HTTP/1.1 {status} OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                socket.write_all(response.as_bytes()).await.unwrap();
            }
        });
        (address, recorded)
    }

    #[tokio::test]
    async fn exchanges_the_identity_token_for_an_access_token_assuming_the_role() {
        let (address, recorded) = serve(vec![(
            200,
            r#"{"access_token":"whop-token-1","expires_in":3600}"#,
        )])
        .await;

        let token = Exchange::new(
            "aurl_xxxxxxxxxxxxx",
            IdentityToken::Value("id-token".to_string()),
        )
        .base_url(format!("{address}/api/v1/"))
        .access_token()
        .await
        .unwrap();

        assert_eq!(token, "whop-token-1");
        let request = recorded.lock().unwrap()[0].clone();
        assert_eq!(
            request.request_line,
            "POST /api/v1/access_tokens/exchange HTTP/1.1"
        );
        let body: serde_json::Value = serde_json::from_str(&request.body).unwrap();
        assert_eq!(
            body,
            serde_json::json!({
                "grant_type": "urn:ietf:params:oauth:grant-type:token-exchange",
                "subject_token_type": "urn:ietf:params:oauth:token-type:id_token",
                "subject_token": "id-token",
                "role_id": "aurl_xxxxxxxxxxxxx",
            })
        );
    }

    #[tokio::test]
    async fn passes_the_requested_expiry() {
        let (address, recorded) = serve(vec![(
            200,
            r#"{"access_token":"whop-token-1","expires_in":7200}"#,
        )])
        .await;
        let expires_at = DateTime::parse_from_rfc3339("2026-10-23T14:00:00Z")
            .unwrap()
            .with_timezone(&Utc);

        Exchange::new(
            "aurl_xxxxxxxxxxxxx",
            IdentityToken::Value("id-token".to_string()),
        )
        .base_url(address)
        .expires_at(expires_at)
        .access_token()
        .await
        .unwrap();

        let body: serde_json::Value =
            serde_json::from_str(&recorded.lock().unwrap()[0].body).unwrap();
        assert_eq!(body["expires_at"], "2026-10-23T14:00:00Z");
    }

    #[tokio::test]
    async fn surfaces_the_api_error_and_code() {
        let (address, _recorded) = serve(vec![(
            400,
            r#"{"error":{"type":"bad_request","message":"No trust linked to this role accepts this token.","code":"oidc_exchange_no_match"}}"#,
        )])
        .await;

        let error = Exchange::new(
            "aurl_xxxxxxxxxxxxx",
            IdentityToken::Value("id-token".to_string()),
        )
        .base_url(address)
        .access_token()
        .await
        .unwrap_err();

        match error {
            OidcError::Refused {
                status,
                code,
                message,
            } => {
                assert_eq!(status, 400);
                assert_eq!(code.as_deref(), Some("oidc_exchange_no_match"));
                assert_eq!(message, "No trust linked to this role accepts this token.");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[tokio::test]
    async fn requires_a_role() {
        let error = access_token("", IdentityToken::Value("id-token".to_string()))
            .await
            .unwrap_err();
        assert!(matches!(error, OidcError::Config(_)));
    }

    #[tokio::test]
    async fn asks_the_google_cloud_metadata_server_for_the_audience() {
        let (address, recorded) = serve(vec![(200, "google-id-token\n")]).await;

        let token = google_cloud_token(
            &reqwest::Client::new(),
            &format!("{address}/identity"),
            "https://example.com",
        )
        .await
        .unwrap();

        assert_eq!(token, "google-id-token");
        let request = recorded.lock().unwrap()[0].clone();
        assert_eq!(
            request.request_line,
            "GET /identity?audience=https%3A%2F%2Fexample.com HTTP/1.1"
        );
        assert_eq!(request.header("Metadata-Flavor"), Some("Google"));
    }

    #[tokio::test]
    async fn google_cloud_asks_the_metadata_server_for_the_default_or_given_audience() {
        let (proxy, recorded) =
            serve(vec![(200, "default-id-token"), (200, "custom-id-token")]).await;
        let http = reqwest::Client::builder()
            .proxy(reqwest::Proxy::http(proxy).unwrap())
            .build()
            .unwrap();

        let default_token = IdentityToken::google_cloud().resolve(&http).await.unwrap();
        let custom_token = IdentityToken::google_cloud_with_audience("https://example.com")
            .resolve(&http)
            .await
            .unwrap();

        assert_eq!(default_token, "default-id-token");
        assert_eq!(custom_token, "custom-id-token");
        let requests = recorded.lock().unwrap().clone();
        assert_eq!(
            requests[0].request_line,
            format!(
                "GET {GOOGLE_METADATA_IDENTITY_URL}?audience=https%3A%2F%2Fapi.whop.com HTTP/1.1"
            )
        );
        assert_eq!(
            requests[1].request_line,
            format!(
                "GET {GOOGLE_METADATA_IDENTITY_URL}?audience=https%3A%2F%2Fexample.com HTTP/1.1"
            )
        );
    }
}
