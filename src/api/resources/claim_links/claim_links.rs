use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ClaimLinksClient {
    pub http_client: HttpClient,
}

impl ClaimLinksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves a funded claim link by ID or by its public claim code. By ID, the caller needs `airdrop_link:basic:read` on the funding account, or must own the funding personal account; `code` and `claim_url` are `null` without `airdrop_link:manage` on the funding account or `payout:withdraw_funds` on the personal account. A claim code previews the sender, amount, expiry, and claim availability without authentication. Treat codes as secrets: anyone holding one can claim after signing in.
    ///
    /// # Arguments
    ///
    /// * `id` - A claim-link ID (`airdrp_…`), or the public claim code from the `c` parameter in the claim URL.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use whop_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Whop::new(config).expect("Failed to build client");
    ///     client.claim_links.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetrieveClaimLinksResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                &format!("claim_links/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Claims a funded link into the authenticated user's personal balance and returns the updated link. Requires a signed-in user and the public claim code; account API keys cannot claim on a recipient's behalf. Each user can claim a link once. Reuse the same `Idempotency-Key` when retrying the same request. On-chain claims may take several minutes to complete.
    ///
    /// # Arguments
    ///
    /// * `id` - The public claim code from the `c` parameter in the claim URL. A claim-link ID does not authorize redemption.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use whop_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Whop::new(config).expect("Failed to build client");
    ///     client.claim_links.claim(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn claim(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ClaimClaimLinksResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-11-12".to_string());
            Some(o)
        };
        let base_url = self
            .http_client
            .config()
            .service_url(|environment| environment.api_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                &format!("claim_links/{}/claim", id),
                None,
                None,
                options,
            )
            .await
    }
}
