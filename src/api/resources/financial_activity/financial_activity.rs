use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FinancialActivityClient {
    pub http_client: HttpClient,
}

impl FinancialActivityClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns an account's or user's activity feed: every movement of money in or out.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The owning account ID (a biz_ identifier). Provide this or user_id.
    /// * `user_id` - The owning user ID (a user_ identifier). Provide this or account_id.
    /// * `include_owned_accounts` - When true, aggregates the authenticated user's personal ledger with the businesses they own (owner role with balance read) into one feed. Requires user_id to be the authenticated user; cannot be combined with account_id or the settlement-date filters. Each returned row includes the owning `account`.
    /// * `include_resource` - Whether to include the `resource` field in the response or not. Consider passing `false` if you need a fast response without as many rich details.
    /// * `line_types` - Optional ledger line categories to include. Some categories (for example `onchain_deposit`, which covers inbound crypto deposits such as MoonPay onramps) are only returned when explicitly requested here.
    /// * `direction` - Optional direction filter. `money_in` returns positive activity and `money_out` returns negative activity.
    /// * `resource_id` - Optional prefixed resource ID. Returns activity associated with that resource.
    /// * `currency` - Optional currency code filter, for example `usd`.
    /// * `posted_after` - Only include rows posted after this ISO 8601 timestamp.
    /// * `posted_before` - Only include rows posted before this ISO 8601 timestamp.
    /// * `available_after` - Only include rows whose funds became withdrawable on or after this `YYYY-MM-DD` settlement date (UTC), distinct from posted_at. Requires currency.
    /// * `available_before` - Only include rows whose funds became withdrawable on or before this `YYYY-MM-DD` settlement date (UTC). Set equal to available_after for a single day. Requires currency.
    /// * `limit` - Maximum number of rows to return.
    /// * `cursor` - Cursor returned by the previous page.
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
    ///         .financial_activity
    ///         .list(
    ///             &FinancialActivityListQueryRequest {
    ///                 account_id: None,
    ///                 user_id: None,
    ///                 include_owned_accounts: None,
    ///                 include_resource: None,
    ///                 line_types: vec![],
    ///                 direction: None,
    ///                 resource_id: None,
    ///                 currency: None,
    ///                 posted_after: None,
    ///                 posted_before: None,
    ///                 available_after: None,
    ///                 available_before: None,
    ///                 limit: None,
    ///                 cursor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &FinancialActivityListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListFinancialActivityResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "financial-activity",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .bool(
                        "include_owned_accounts",
                        request.include_owned_accounts.clone(),
                    )
                    .bool("include_resource", request.include_resource.clone())
                    .serialize_array("line_types", request.line_types.clone())
                    .serialize("direction", request.direction.clone())
                    .string("resource_id", request.resource_id.clone())
                    .string("currency", request.currency.clone())
                    .datetime("posted_after", request.posted_after.clone())
                    .datetime("posted_before", request.posted_before.clone())
                    .date("available_after", request.available_after.clone())
                    .date("available_before", request.available_before.clone())
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }
}
