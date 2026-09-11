use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CashbackRulesClient {
    pub http_client: HttpClient,
}

impl CashbackRulesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a future-dated card cashback rule funded by the authenticated platform account. Requires payout:transfer_funds. Both the raw merchant name and four-digit MCC are required. Optionally limit the rule to one direct connected account. The funding account is derived from the credential and cannot be supplied. Creation does not transfer funds. Supports Idempotency-Key for safe retries.
    ///
    /// # Arguments
    ///
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
    ///         .cashback_rules
    ///         .create(
    ///             &CreateCashbackRulesRequest {
    ///                 merchant_category_code: "5734".to_string(),
    ///                 merchant_name: "ACME SOFTWARE".to_string(),
    ///                 rate_bps: 500,
    ///                 starts_at: DateTime::parse_from_rfc3339("2026-01-01T12:00:00Z").unwrap(),
    ///                 description: None,
    ///                 expires_at: None,
    ///                 scoped_account_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateCashbackRulesRequest,
        options: Option<RequestOptions>,
    ) -> Result<CashbackRule, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "cashback_rule",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists all cashback rules funded by the authenticated platform account. Includes scheduled, expired, and discarded rules. Requires payout:transfer:read. Account-scoped credentials are required; there is no caller-supplied funding-account filter.
    ///
    /// # Arguments
    ///
    /// * `first` - Number of rules to return from the start of the page.
    /// * `after` - Return rules after this cursor.
    /// * `last` - Number of rules to return from the end of the page.
    /// * `before` - Return rules before this cursor.
    /// * `order` - Field to sort by. Defaults to created_at.
    /// * `direction` - Sort direction. Defaults to desc.
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
    ///         .cashback_rules
    ///         .list(
    ///             &CashbackRulesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &CashbackRulesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCashbackRulesResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "cashback_rules",
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates a cashback rule funded by the authenticated platform account. Requires payout:transfer_funds. Only merchant_name, merchant_category_code, description, and expires_at can change; starts_at, rate_bps, funding_account_id, and scoped_account_id are immutable. Omitted fields stay unchanged. Scheduled, active, and expired rules can be updated; discarded rules cannot. Updating a rule does not transfer funds.
    ///
    /// # Arguments
    ///
    /// * `id` - ID of the cashback rule, prefixed cicbr_.
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
    ///         .cashback_rules
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateCashbackRulesRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateCashbackRulesRequest,
        options: Option<RequestOptions>,
    ) -> Result<CashbackRule, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-09-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("cashback_rules/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
