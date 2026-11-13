use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub mod customer_tracking;
pub use customer_tracking::CustomerTrackingClient;
pub mod fees;
pub use fees::FeesClient;
pub mod financing_providers;
pub use financing_providers::FinancingProvidersClient;
pub mod preferences;
pub use preferences::PreferencesClient;
pub mod reserves;
pub use reserves::ReservesClient;
pub struct AccountsClient {
    pub http_client: HttpClient,
    pub customer_tracking: CustomerTrackingClient,
    pub fees: FeesClient,
    pub financing_providers: FinancingProvidersClient,
    pub preferences: PreferencesClient,
    pub reserves: ReservesClient,
}

impl AccountsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            customer_tracking: CustomerTrackingClient::new(config.clone())?,
            fees: FeesClient::new(config.clone())?,
            financing_providers: FinancingProvidersClient::new(config.clone())?,
            preferences: PreferencesClient::new(config.clone())?,
            reserves: ReservesClient::new(config.clone())?,
        })
    }

    /// Lists accounts visible to the credential. User tokens return the user's business accounts; Account API keys return the requesting account and its subaccounts.
    ///
    /// # Arguments
    ///
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `order` - The field to sort accounts by. `volume` requires `stats:read` on the parent account.
    /// * `direction` - Sort direction.
    /// * `status` - Return only accounts with this status: `active` (includes accounts that have not entered payments review) or `suspended`.
    /// * `query` - Free-text filter on account title or ID. `%` and `_` match literally.
    /// * `created_after` - Return only accounts created after this ISO 8601 timestamp.
    /// * `created_before` - Return only accounts created before this ISO 8601 timestamp.
    /// * `external_id` - Return only the subaccount with this `external_id`, matched exactly and case-sensitively. End it with `%` to return every subaccount whose `external_id` starts with the text before it, such as `shopline:%`. `%` is accepted only at the end, because anywhere else it would scan all your subaccounts. `_` always matches literally.
    /// * `volume_min` - Return only accounts whose lifetime USD volume is at least this value. Requires `stats:read` on the parent account.
    /// * `volume_max` - Return only accounts whose lifetime USD volume is at most this value. Requires `stats:read` on the parent account.
    /// * `parent_account_id` - For platforms: the parent account ID whose direct subaccounts to return. Requires `payout:account:read` on the parent account.
    /// * `owner_id` - Return only accounts owned by this user (`user_...`), the account's `owner`. Cannot be combined with `parent_account_id`.
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
    ///         .accounts
    ///         .list(
    ///             &AccountsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AccountsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAccountsResponse, ApiError> {
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
                "accounts",
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize("status", request.status.clone())
                    .structured_query("query", request.query.clone())
                    .datetime("created_after", request.created_after.clone())
                    .datetime("created_before", request.created_before.clone())
                    .string("external_id", request.external_id.clone())
                    .float("volume_min", request.volume_min.clone())
                    .float("volume_max", request.volume_max.clone())
                    .string("parent_account_id", request.parent_account_id.clone())
                    .string("owner_id", request.owner_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &AccountsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .serialize("status", request.status.clone())
            .structured_query("query", request.query.clone())
            .datetime("created_after", request.created_after.clone())
            .datetime("created_before", request.created_before.clone())
            .string("external_id", request.external_id.clone())
            .float("volume_min", request.volume_min.clone())
            .float("volume_max", request.volume_max.clone())
            .string("parent_account_id", request.parent_account_id.clone())
            .string("owner_id", request.owner_id.clone())
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
                            "accounts",
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

    /// Creates an account. User tokens create business accounts; Account API keys create subaccounts. Tax fields (`tax_remitted_by`, `tax_type`, `product_tax_code_id`, `business_address`, `tax_identifiers`) are configured with Update Account, not at creation.
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
    ///         .accounts
    ///         .create(
    ///             &CreateAccountsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
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
                "accounts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the account associated with the current Account API key.
    ///
    /// # Arguments
    ///
    /// * `include_trading` - Also return the trading account under `trading`: its address, a snapshot of its margin and open positions, and its Hyperliquid WebSocket subscriptions. Requires crypto_wallet:trade:read, crypto_wallet:trade, or crypto_wallet:manage permission and an Ethereum wallet. Otherwise `trading` is `null`.
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
    ///         .accounts
    ///         .me(
    ///             &AccountsMeQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn me(
        &self,
        request: &AccountsMeQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
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
                "accounts/me",
                None,
                QueryBuilder::new()
                    .bool("include_trading", request.include_trading.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Finds accounts by name among the accounts the credential can list. Support searches cover every company and return only the 50 best matches.
    ///
    /// # Arguments
    ///
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    /// * `last` - Number of results to return from the end of the range.
    /// * `before` - Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    /// * `query` - The account name to search for.
    /// * `order` - Sort field. Without it, support searches return the best match first and other searches return the newest account first.
    /// * `direction` - Sort direction.
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
    ///         .accounts
    ///         .search(
    ///             &SearchQueryRequest {
    ///                 query: "query".to_string(),
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///                 order: None,
    ///                 direction: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search(
        &self,
        request: &SearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchAccountsResponse, ApiError> {
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
                "accounts/search",
                None,
                QueryBuilder::new()
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .int("last", request.last.clone())
                    .string("before", request.before.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn search_paginated(
        &self,
        request: &SearchQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .string("before", request.before.clone())
            .structured_query("query", request.query.clone())
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
                            "accounts/search",
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

    /// Retrieves an account visible to the credential by ID or public route, including its crypto wallet.
    ///
    /// # Arguments
    ///
    /// * `id` - Account ID, prefixed `biz_`, its public route, or `me` for the account associated with the current Account API key. User tokens have no single account, so they cannot use `me`.
    /// * `include_trading` - Also return the trading account under `trading`: its address, a snapshot of its margin and open positions, and its Hyperliquid WebSocket subscriptions. Requires crypto_wallet:trade:read, crypto_wallet:trade, or crypto_wallet:manage permission and an Ethereum wallet. Otherwise `trading` is `null`.
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
    ///         .accounts
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &AccountsRetrieveQueryRequest {
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
        request: &AccountsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
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
                &format!("accounts/{}", id),
                None,
                QueryBuilder::new()
                    .bool("include_trading", request.include_trading.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a subaccount directly owned by the authenticated platform account. The account must have no settled, pending, or reserved balance in any currency and no active, trialing, or past-due memberships. The account stops resolving immediately, and its products, plans, and team access are removed in the background; payment history is retained. Deletion cannot be undone through the API. This cannot delete the platform account itself or an account owned by another platform.
    ///
    /// # Arguments
    ///
    /// * `id` - Subaccount ID, prefixed `biz_`.
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
    ///     client.accounts.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteAccountsResponse, ApiError> {
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
                Method::DELETE,
                &format!("accounts/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an account. User tokens can update business accounts; Account API keys can update subaccounts.
    ///
    /// # Arguments
    ///
    /// * `id` - Account ID, prefixed `biz_`. The reserved id `me` resolves to the requesting account, which an Account API key cannot edit.
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
    ///         .accounts
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateAccountsRequest {
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
        request: &UpdateAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
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
                &format!("accounts/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Starts an LLC or C-Corp formation for a business account. The application is validated and the response returns a hosted checkout URL; once paid, the filing is submitted. Track progress through the account's [`company_formation`](/api-reference/accounts/retrieve-account) field on Retrieve Account.
    ///
    /// # Arguments
    ///
    /// * `id` - Account ID, prefixed `biz_`.
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
    ///         .accounts
    ///         .form_company(
    ///             &"id".to_string(),
    ///             &FormCompanyAccountsRequest {
    ///                 business_address: Some(FormCompanyAccountsRequestBusinessAddress {
    ///                     city: "Austin".to_string(),
    ///                     country: "US".to_string(),
    ///                     line1: "4180 Burnet Rd".to_string(),
    ///                     line2: Some("Suite 2".to_string()),
    ///                     postal_code: "78756".to_string(),
    ///                     state: "TX".to_string(),
    ///                     ..Default::default()
    ///                 }),
    ///                 business_name: "Shine Time Auto Detailing".to_string(),
    ///                 business_phone: Some("+15125550142".to_string()),
    ///                 business_website: Some("https://shinetime.example".to_string()),
    ///                 entity_suffix: Some(FormCompanyAccountsRequestEntitySuffix::Llc),
    ///                 entity_type: Some(FormCompanyAccountsRequestEntityType::Llc),
    ///                 expedite_ein: Some(true),
    ///                 formation_state: FormCompanyAccountsRequestFormationState::Wy,
    ///                 founders: vec![FormCompanyAccountsRequestFoundersItem {
    ///                     address: FormCompanyAccountsRequestFoundersItemAddress {
    ///                         city: "Austin".to_string(),
    ///                         country: "US".to_string(),
    ///                         line1: "907 Ridgemont Dr".to_string(),
    ///                         line2: Some("Apt 4".to_string()),
    ///                         postal_code: "78704".to_string(),
    ///                         state: "TX".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     date_of_birth: Some("1988-03-14".to_string()),
    ///                     email: "marcus@shinetime.example".to_string(),
    ///                     first_name: "Marcus".to_string(),
    ///                     is_primary: true,
    ///                     last_name: "Webb".to_string(),
    ///                     ownership_percentage: Some(100.0),
    ///                     phone: "+15125550142".to_string(),
    ///                     roles: Some(vec![
    ///                         FormCompanyAccountsRequestFoundersItemRolesItem::President,
    ///                     ]),
    ///                     ssn: Some("123-45-6789".to_string()),
    ///                     ..Default::default()
    ///                 }],
    ///                 share_structure: Some(FormCompanyAccountsRequestShareStructure {
    ///                     number_of_shares: 123,
    ///                     value: 123.0,
    ///                     ..Default::default()
    ///                 }),
    ///                 use_registered_agent: Some(true),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn form_company(
        &self,
        id: &str,
        request: &FormCompanyAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<FormCompanyAccountsResponse, ApiError> {
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
                &format!("accounts/{}/form_company", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Queues one background retry of the account's failed ads payments across its campaigns, using the account's configured ads payment methods. A queued response does not mean payment succeeded; read each campaign's `delivery_status` and `issues` for the outcome. Successful settlement clears the payment block without changing a configured `active` or `paused` status; a legacy `payment_failed` status becomes `paused`. Returns an error when the account has no failed ads payments, or while a previous retry for the account is queued or running.
    ///
    /// # Arguments
    ///
    /// * `id` - Account ID, prefixed `biz_`.
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
    ///         .accounts
    ///         .retry_ads_payment(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retry_ads_payment(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<RetryAdsPaymentAccountsResponse, ApiError> {
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
                &format!("accounts/{}/retry_ads_payment", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Suspends a subaccount directly owned by the authenticated platform account. This cannot suspend the platform account itself or an account owned by another platform.
    ///
    /// # Arguments
    ///
    /// * `id` - Subaccount ID, prefixed `biz_`.
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
    ///     client.accounts.suspend(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn suspend(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Account, ApiError> {
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
                &format!("accounts/{}/suspend", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Transfers ownership of the account to another user, identified by user ID or email address. If the recipient already holds the owner role, ownership moves immediately; otherwise they get an invite and ownership moves when they accept. The Whop CLI and MCP can hand off a business their user owns as a partner (`as_partner: true`) with the `partner:handoff` scope; other OAuth apps cannot transfer ownership.
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
    ///         .accounts
    ///         .transfer_ownership(
    ///             &"id".to_string(),
    ///             &TransferOwnershipAccountsRequest {
    ///                 identifier: "marcus@shinetime.example".to_string(),
    ///                 as_partner: None,
    ///                 authorized_user_id: None,
    ///                 member_id: None,
    ///                 message: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn transfer_ownership(
        &self,
        id: &str,
        request: &TransferOwnershipAccountsRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransferOwnershipAccountsResponse, ApiError> {
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
                &format!("accounts/{}/transfer_ownership", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
