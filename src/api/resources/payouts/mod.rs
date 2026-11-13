use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub mod methods;
pub use methods::MethodsClient;
pub mod paypal_link_sessions;
pub use paypal_link_sessions::PaypalLinkSessionsClient;
pub mod supported_methods;
pub use supported_methods::SupportedMethodsClient;
pub struct PayoutsClient {
    pub http_client: HttpClient,
    pub methods: MethodsClient,
    pub paypal_link_sessions: PaypalLinkSessionsClient,
    pub supported_methods: SupportedMethodsClient,
}

impl PayoutsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            methods: MethodsClient::new(config.clone())?,
            paypal_link_sessions: PaypalLinkSessionsClient::new(config.clone())?,
            supported_methods: SupportedMethodsClient::new(config.clone())?,
        })
    }

    /// Lists an account's or user's payouts, newest first.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The owning account ID (a biz_ identifier). Provide this or user_id.
    /// * `user_id` - The owning user ID (a user_ identifier). Provide this or account_id.
    /// * `currency` - Optional currency code filter, for example `usd`.
    /// * `status` - Filter to payouts whose `status` reads this word, matching exactly what this version displays — `reversed` finds settled payouts the bank later returned. Requires Api-Version-Date 2026-08-21 or later.
    /// * `source` - Filter by how the payout was created. A payout with no recorded source never matches.
    /// * `payout_method_id` - Filter to payouts sent to one saved payout method (a pytk_ identifier). An unknown id matches nothing.
    /// * `created_before` - Only payouts created before this ISO 8601 time (exclusive).
    /// * `created_after` - Only payouts created at or after this ISO 8601 time (inclusive).
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
    ///         .payouts
    ///         .list(
    ///             &PayoutsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &PayoutsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPayoutsResponse, ApiError> {
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
                "payouts",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string("currency", request.currency.clone())
                    .serialize("status", request.status.clone())
                    .serialize("source", request.source.clone())
                    .string("payout_method_id", request.payout_method_id.clone())
                    .datetime("created_before", request.created_before.clone())
                    .datetime("created_after", request.created_after.clone())
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
        request: &PayoutsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .string("user_id", request.user_id.clone())
            .string("currency", request.currency.clone())
            .serialize("status", request.status.clone())
            .serialize("source", request.source.clone())
            .string("payout_method_id", request.payout_method_id.clone())
            .datetime("created_before", request.created_before.clone())
            .datetime("created_after", request.created_after.clone())
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
                            "payouts",
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

    /// Sends money from an account or user balance to a saved payout method for that owner. First create a quote with POST /payouts/quotes, then submit its quote_token with the same payout details before it expires.
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
    ///         .payouts
    ///         .create(
    ///             &CreatePayoutsRequest {
    ///                 amount: 50.0,
    ///                 payout_method_id: "potk_xxxxxxxxxxxxxx".to_string(),
    ///                 quote_token: "signed-payout-quote".to_string(),
    ///                 account_id: None,
    ///                 acknowledge_bank_warning: None,
    ///                 currency: None,
    ///                 metadata: None,
    ///                 notes: None,
    ///                 payment_method_id: None,
    ///                 platform_covers_fees: None,
    ///                 speed: None,
    ///                 statement_descriptor: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreatePayoutsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatePayoutsResponse, ApiError> {
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
                "payouts",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a short-lived, provider-backed quote of a payout's fee, exchange rate, and destination amount. No funds move until you submit the returned `quote_token` to `POST /payouts`.
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
    ///         .payouts
    ///         .create_quote(
    ///             &CreateQuotePayoutsRequest {
    ///                 payout_method_id: "potk_xxxxxxxxxxxxxx".to_string(),
    ///                 account_id: None,
    ///                 amount: None,
    ///                 currency: None,
    ///                 payment_method_id: None,
    ///                 platform_covers_fees: None,
    ///                 recipient_amount: None,
    ///                 speed: None,
    ///                 statement_descriptor: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_quote(
        &self,
        request: &CreateQuotePayoutsRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateQuotePayoutsResponse, ApiError> {
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
                "payouts/quotes",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a payout by its `wdrl_` ID, or by the `cofr_` conversion request ID a payout carries as `payout_request_id`. Authentication is optional: anyone with the ID can view tracking details, including notes, trace code, exchange rate, and payout request ID, while accounting fields require `payout:withdrawal:read` on the account or user that owns the payout. A supplied invalid credential returns 401.
    ///
    /// # Arguments
    ///
    /// * `id` - Payout ID, prefixed `wdrl_` for a payout returned by `GET /payouts` or `cofr_` for the payout request returned by `POST /payouts`.
    /// * `account_id` - Optional owning account ID, prefixed `biz_`. The payout ID identifies its ledger. If supplied, this must match the owner and cannot be combined with `user_id`.
    /// * `user_id` - Optional owning user ID, prefixed `user_`. The payout ID identifies its ledger. If supplied, this must match the owner and cannot be combined with `account_id`.
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
    ///         .payouts
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &PayoutsRetrieveQueryRequest {
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
        request: &PayoutsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RetrievePayoutsResponse, ApiError> {
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
                &format!("payouts/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Cancels a payout before it is sent to the payout provider, or asks the provider to stop one already sent. While the payout is `in_review`, or `processing` with nothing sent yet (see `cancelable`), it is canceled and the funds, fees included, return to the balance. Once sent, the response is 202 with a `cancellation` object, and the payout reads `canceled` only if the provider confirms it stopped. A `requested` payout returns 409 until it reaches review, a payout the provider can no longer stop returns 409 with error type `not_cancelable`, and canceling an already-canceled payout returns it unchanged.
    ///
    /// # Arguments
    ///
    /// * `id` - Payout ID, prefixed `wdrl_`, or the `cofr_` payout request ID returned by `POST /payouts` — both cancel the same payout.
    /// * `user_id` - Owning user ID, prefixed `user_`. Provide exactly one of `account_id` or `user_id`.
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
    ///         .payouts
    ///         .cancel(
    ///             &"id".to_string(),
    ///             &CancelQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        request: &CancelQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CancelPayoutsResponse, ApiError> {
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
                &format!("payouts/{}/cancel", id),
                None,
                QueryBuilder::new()
                    .string("user_id", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
