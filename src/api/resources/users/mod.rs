use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub mod oauth_grants;
pub use oauth_grants::OauthGrantsClient;
pub mod passkeys;
pub use passkeys::PasskeysClient;
pub mod preferences;
pub use preferences::PreferencesClient2;
pub mod fees;
pub use fees::FeesClient2;
pub struct UsersClient {
    pub http_client: HttpClient,
    pub oauth_grants: OauthGrantsClient,
    pub passkeys: PasskeysClient,
    pub preferences: PreferencesClient2,
    pub fees: FeesClient2,
}

impl UsersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            oauth_grants: OauthGrantsClient::new(config.clone())?,
            passkeys: PasskeysClient::new(config.clone())?,
            preferences: PreferencesClient2::new(config.clone())?,
            fees: FeesClient2::new(config.clone())?,
        })
    }

    /// Searches for users by name or username, ranked by social proximity to the authenticated user. Without a `query`, returns the user's most recently followed users.
    ///
    /// # Arguments
    ///
    /// * `context_channel_id` - Prioritize people relevant to this channel for mentions. Results remain global; channel access is required.
    /// * `query` - A search term to filter users by name or username.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
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
    ///         .list(
    ///             &UsersListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &UsersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListUsersResponse, ApiError> {
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
                "users",
                None,
                QueryBuilder::new()
                    .string("context_channel_id", request.context_channel_id.clone())
                    .structured_query("query", request.query.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &UsersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("context_channel_id", request.context_channel_id.clone())
            .structured_query("query", request.query.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
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
                            "users",
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

    /// Retrieves the authenticated user. Same as `GET /users/:id` with the reserved id `me`: the self view, where self-only fields such as `email`, `balance`, and `cards` can be populated.
    ///
    /// # Arguments
    ///
    /// * `include_trading` - Also return the trading account under `trading`: its address, a snapshot of its margin and open positions, and its Hyperliquid WebSocket subscriptions. Only honored on the self view (me) with crypto_wallet:trade:read, crypto_wallet:trade, or crypto_wallet:manage permission and an Ethereum wallet.
    /// * `account_id` - When set, returns your account-specific profile overrides for this account.
    /// * `include_balance` - Compute live wallet and owned-account balances (default true). Set false for identity-only reads. Ignored for callers without balance-read scope.
    /// * `include_balance_history` - Also compute your balance history (opt-in; runs a heavier query). Ignored for callers without balance-read scope.
    /// * `from` - Balance-history window start, ISO 8601 date or datetime. Defaults to 30 days ago. Only used with `include_balance_history`.
    /// * `to` - Balance-history window end, ISO 8601 date or datetime. Defaults to now. Only used with `include_balance_history`.
    /// * `interval` - Balance-history point granularity. Defaults to `day`. Only used with `include_balance_history`.
    /// * `time_zone` - IANA time zone the balance-history points are bucketed in. Defaults to `UTC`. Only used with `include_balance_history`.
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
    ///         .me(
    ///             &UsersMeQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn me(
        &self,
        request: &UsersMeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
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
                "users/me",
                None,
                QueryBuilder::new()
                    .bool("include_trading", request.include_trading.clone())
                    .string("account_id", request.account_id.clone())
                    .bool("include_balance", request.include_balance.clone())
                    .bool(
                        "include_balance_history",
                        request.include_balance_history.clone(),
                    )
                    .string("from", request.from.clone())
                    .string("to", request.to.clone())
                    .serialize("interval", request.interval.clone())
                    .string("time_zone", request.time_zone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates the authenticated user's global profile, or their profile override for an account when `account_id` is given. Not available to API keys.
    ///
    /// # Arguments
    ///
    /// * `account_id` - When set, updates the authenticated user's profile override for this account instead of their global profile.
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
    ///         .update_me(
    ///             &UpdateMeUsersRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_me(
        &self,
        request: &UpdateMeUsersRequest,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
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
                Method::PATCH,
                "users/me",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a user by `user_` tag or username, or the authenticated user with the reserved id `me`. Self-only fields such as `email`, `balance`, and `earnings_usd` are populated only when the id is `me`, and are always `null` when addressing a user by tag or username.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID (prefixed `user_`), username, or `me` for the authenticated user.
    /// * `include_trading` - Also return the trading account under `trading`: its address, a snapshot of its margin and open positions, and its Hyperliquid WebSocket subscriptions. Only honored on the self view (me) with crypto_wallet:trade:read, crypto_wallet:trade, or crypto_wallet:manage permission and an Ethereum wallet.
    /// * `account_id` - When set, returns the user's account-specific profile overrides for this account.
    /// * `include_balance` - Compute live wallet and owned-account balances on the self view (default true). Set false for identity-only reads. Ignored when the id is not `me` or the caller lacks balance-read scope.
    /// * `include_balance_history` - Also compute your balance history (opt-in; runs a heavier query). Only applies when the id is `me`; ignored for callers without balance-read scope.
    /// * `from` - Balance-history window start, ISO 8601 date or datetime. Defaults to 30 days ago. Only used with `include_balance_history`.
    /// * `to` - Balance-history window end, ISO 8601 date or datetime. Defaults to now. Only used with `include_balance_history`.
    /// * `interval` - Balance-history point granularity. Defaults to `day`. Only used with `include_balance_history`.
    /// * `time_zone` - IANA time zone the balance-history points are bucketed in. Defaults to `UTC`. Only used with `include_balance_history`.
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
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &UsersRetrieveQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        request: &UsersRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
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
                &format!("users/{}", id),
                None,
                QueryBuilder::new()
                    .bool("include_trading", request.include_trading.clone())
                    .string("account_id", request.account_id.clone())
                    .bool("include_balance", request.include_balance.clone())
                    .bool(
                        "include_balance_history",
                        request.include_balance_history.clone(),
                    )
                    .string("from", request.from.clone())
                    .string("to", request.to.clone())
                    .serialize("interval", request.interval.clone())
                    .string("time_zone", request.time_zone.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates a user, addressed by `user_` tag, username, or the reserved id `me` for the authenticated user. A user token updates their own global profile; an API key updates the user's profile override for the account in `account_id`.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID (prefixed `user_`), username, or `me` for the authenticated user.
    /// * `account_id` - The account whose profile override to update. Required for API key callers.
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
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateUsersRequest {
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
        request: &UpdateUsersRequest,
        options: Option<RequestOptions>,
    ) -> Result<User, ApiError> {
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
                Method::PATCH,
                &format!("users/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Checks whether a user has access to an account, product, or experience the caller can reach.
    ///
    /// # Arguments
    ///
    /// * `id` - The user_ tag or username to check access for.
    /// * `resource_id` - An account (biz_), product (prod_), or experience (exp_) ID.
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
    ///         .check_access(&"id".to_string(), &"resource_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn check_access(
        &self,
        id: &str,
        resource_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CheckAccessUsersResponse, ApiError> {
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
                &format!("users/{}/access/{}", id, resource_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists the recommended actions computed for the authenticated user, such as starting a business. You can only list your own recommended actions.
    ///
    /// # Arguments
    ///
    /// * `id` - `me`, or the authenticated user's own `user_` tag or username.
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
    ///         .recommend_actions(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn recommend_actions(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RecommendActionsUsersResponse, ApiError> {
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
                &format!("users/{}/recommend_actions", id),
                None,
                None,
                options,
            )
            .await
    }
}
