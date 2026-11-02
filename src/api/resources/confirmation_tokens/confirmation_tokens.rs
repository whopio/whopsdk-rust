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

    /// Retrieves a token's display-safe preview — never the underlying payment credential. Public and rate-limited: the account_id query param must match the account the token was minted for.
    ///
    /// # Arguments
    ///
    /// * `id` - Confirmation token ID, prefixed `ctok_`.
    /// * `account_id` - The account (biz_) the token was minted for.
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
                .or_insert_with(|| "2026-09-15".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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
