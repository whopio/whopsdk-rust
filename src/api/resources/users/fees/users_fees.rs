use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct FeesClient2 {
    pub http_client: HttpClient,
}

impl FeesClient2 {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves the fees charged when the user moves money with their personal balance: withdrawals, transfers, bank deposits, and funding cards from pending balance. Each fee has the same shape as in Retrieve Account Fees. You can only retrieve your own fees, and they are read-only.
    ///
    /// # Arguments
    ///
    /// * `user_id` - `me`, or the authenticated user's own `user_` tag or username.
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
    ///     client
    ///         .users
    ///         .fees
    ///         .retrieve(&"user_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<UserFees, ApiError> {
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
                &format!("users/{}/fees", user_id),
                None,
                None,
                options,
            )
            .await
    }
}
