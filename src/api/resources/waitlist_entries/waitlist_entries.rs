use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct WaitlistEntriesClient {
    pub http_client: HttpClient,
}

impl WaitlistEntriesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the waitlist signups you can see. `waitlist_entry:read` returns your own signups, and `plan:waitlist:read` returns signups to the seller accounts you are authorized on; with both, you get both sets. Signups submitted on behalf of an account are not in this list; pass `buyer_account_id` to list them instead, which needs `waitlist_entry:read` on that account. Account credentials see the signups submitted to their own account, and pass their own account as `buyer_account_id` to see the ones it submitted.
    ///
    /// # Arguments
    ///
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `variant_id` - Only return signups for this variant, prefixed `plan_`.
    /// * `account_id` - Only return signups submitted to this seller account, prefixed `biz_`.
    /// * `product_id` - Only return signups for variants on this product, prefixed `prod_`.
    /// * `buyer_account_id` - List the signups submitted on behalf of this account, prefixed `biz_`, instead of your own and seller signups. Requires `waitlist_entry:read` on the account. The other filters still apply.
    /// * `status` - Only return signups in this state. Canceled signups are returned only when `status` is `canceled`.
    /// * `query` - Search signups by the user's name or username, or the name of the account they were submitted on behalf of. An account ID, prefixed `biz_`, matches signups submitted on behalf of that account, and a user ID, prefixed `user_`, or an exact email address matches that user's signups.
    /// * `created_before` - Only return signups submitted at or before this ISO 8601 timestamp.
    /// * `created_after` - Only return signups submitted at or after this ISO 8601 timestamp.
    /// * `order` - The field to sort results by. Defaults to `created_at`.
    /// * `direction` - The sort direction for results. Defaults to descending.
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
    ///         .waitlist_entries
    ///         .list(
    ///             &WaitlistEntriesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &WaitlistEntriesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListWaitlistEntriesResponse, ApiError> {
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
                "waitlist_entries",
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .string("variant_id", request.variant_id.clone())
                    .string("account_id", request.account_id.clone())
                    .string("product_id", request.product_id.clone())
                    .string("buyer_account_id", request.buyer_account_id.clone())
                    .serialize("status", request.status.clone())
                    .structured_query("query", request.query.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &WaitlistEntriesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .string("variant_id", request.variant_id.clone())
            .string("account_id", request.account_id.clone())
            .string("product_id", request.product_id.clone())
            .string("buyer_account_id", request.buyer_account_id.clone())
            .serialize("status", request.status.clone())
            .structured_query("query", request.query.clone())
            .string("created_before", request.created_before.clone())
            .string("created_after", request.created_after.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .build();
        let options_clone = options.clone();

        AsyncPaginator::new(
            http_client,
            move |client, cursor_value| {
                let mut query_params: Vec<(String, String)> =
                    base_query_params.clone().unwrap_or_default();
                if let Some(cursor) = cursor_value {
                    // Add cursor parameter based on pagination configuration
                    query_params.push(("after".to_string(), cursor));
                }
                let options_for_request = options_clone.clone();

                // Clone captured variables to move into the async block

                Box::pin(async move {
                    let raw_response = client
                        .execute_request_raw::<serde_json::Value>(
                            Method::GET,
                            "waitlist_entries",
                            None,
                            Some(query_params),
                            options_for_request,
                        )
                        .await?;
                    let response = raw_response.body;

                    // Extract pagination info from response
                    // Generic field extraction using pagination configuration
                    let items: Vec<serde_json::Value> = response
                        .get("data")
                        .and_then(|v| v.as_array())
                        .map(|arr| arr.clone())
                        .unwrap_or_default();

                    let next_cursor: Option<String> = response
                        .get("page_info")
                        .and_then(|v| v.get("end_cursor"))
                        .and_then(|v| v.as_str().map(|s| s.to_string()));
                    let has_next_page = next_cursor.is_some();

                    Ok(PaginationResult {
                        items,
                        next_cursor,
                        has_next_page,
                        response: Some(response),
                        status_code: raw_response.status_code,
                        headers: raw_response.headers,
                    })
                })
            },
            None, // Start with no cursor
        )
    }

    /// Joins a free waitlist variant as the authenticated user, or on behalf of an account with `buyer_account_id`. Requires `waitlist_entry:create`, on that account when one is given. Joining again returns the existing pending signup, or the approved one while its membership is valid. Paid variants are rejected; joining collects no payment method and grants no membership.
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
    ///         .waitlist_entries
    ///         .create(
    ///             &CreateWaitlistEntriesRequest {
    ///                 variant_id: "plan_xxxxxxxxxxxxxx".to_string(),
    ///                 buyer_account_id: None,
    ///                 custom_field_responses: None,
    ///                 metadata: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateWaitlistEntriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
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
                "waitlist_entries",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Queues approval of every pending signup for an account, optionally narrowed to a variant. Requires `plan:waitlist:manage`. Paid signups may charge saved payment methods. Approval runs asynchronously: list signups with `status` set to `pending` to follow progress, and retrieve a signup to read its outcome. Signups created after this request are excluded.
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
    ///         .waitlist_entries
    ///         .approve_all(
    ///             &ApproveAllWaitlistEntriesRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 variant_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn approve_all(
        &self,
        request: &ApproveAllWaitlistEntriesRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApproveAllWaitlistEntriesResponse, ApiError> {
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
                "waitlist_entries/approve_all",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves your own signup or one submitted on behalf of an account you hold `waitlist_entry:read` on, or one submitted to an account you can read, with `plan:waitlist:read`.
    ///
    /// # Arguments
    ///
    /// * `id` - Waitlist signup ID, prefixed `entry_`.
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
    ///         .waitlist_entries
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
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
                &format!("waitlist_entries/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Queues approval of a pending signup. Requires `plan:waitlist:manage` on its seller account. Paid signups may charge their saved payment method. Returns the signup's current state; retrieve it to read `status` and `approval_failure_reason` after processing.
    ///
    /// # Arguments
    ///
    /// * `id` - Waitlist signup ID, prefixed `entry_`.
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
    ///         .waitlist_entries
    ///         .approve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn approve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
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
                &format!("waitlist_entries/{}/approve", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Withdraws a pending signup: your own, or one submitted on behalf of an account you hold `waitlist_entry:cancel` on. Requires `waitlist_entry:cancel`, on that account for an account's signup. Does not cancel an approved membership.
    ///
    /// # Arguments
    ///
    /// * `id` - Waitlist signup ID, prefixed `entry_`.
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
    ///         .waitlist_entries
    ///         .cancel(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
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
                &format!("waitlist_entries/{}/cancel", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Denies a pending signup. Requires `plan:waitlist:manage` on its seller account.
    ///
    /// # Arguments
    ///
    /// * `id` - Waitlist signup ID, prefixed `entry_`.
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
    ///     client.waitlist_entries.deny(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn deny(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WaitlistEntry, ApiError> {
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
                &format!("waitlist_entries/{}/deny", id),
                None,
                None,
                options,
            )
            .await
    }
}
