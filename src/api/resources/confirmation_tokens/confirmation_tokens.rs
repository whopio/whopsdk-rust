use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ConfirmationTokensClient {
    pub http_client: HttpClient,
}

impl ConfirmationTokensClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves a confirmation token's payment method and billing details, never the underlying payment credential, to display what the buyer chose or check that the token is still usable. Requires no authentication and is rate-limited.
    ///
    /// # Arguments
    ///
    /// * `id` - Confirmation token ID, prefixed `ctok_`.
    /// * `account_id` - The account the token was minted for, prefixed `biz_`. It must match the token's account.
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
    ///         .confirmation_tokens
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &ConfirmationTokensRetrieveQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        request: &ConfirmationTokensRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConfirmationToken, ApiError> {
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
                &format!("confirmation_tokens/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
