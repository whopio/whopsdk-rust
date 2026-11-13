use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub mod candles;
pub use candles::CandlesClient;
pub mod markets;
pub use markets::MarketsClient;
pub struct TradesClient {
    pub http_client: HttpClient,
    pub candles: CandlesClient,
    pub markets: MarketsClient,
}

impl TradesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            candles: CandlesClient::new(config.clone())?,
            markets: MarketsClient::new(config.clone())?,
        })
    }

    /// Lists trades you can access, newest first. User credentials see their own trades and those of accounts they belong to, including subaccounts; account credentials see their account and its subaccounts.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Only return trades for this account or user, prefixed `biz_` or `user_`.
    /// * `status` - Only return trades with this status.
    /// * `type_` - Only return trades of this type.
    /// * `order` - Field to sort by.
    /// * `direction` - Sort direction.
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
    ///         .trades
    ///         .list(
    ///             &TradesListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &TradesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListTradesResponse, ApiError> {
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
                "trades",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .serialize("type", request.r#type.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
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
        request: &TradesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .serialize("status", request.status.clone())
            .serialize("type", request.r#type.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
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
                            "trades",
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

    /// Places an order, closes a perpetual position, or deposits to or withdraws from the trading balance, from the Whop-managed wallet of an account or user. Answers `201` with the trade in `pending`; it runs in the background, so read it with `GET /trades/:id` until its `status` is `completed`, `failed` or `in_review`. Trades run alongside each other, except a second order on one market at a different leverage, which answers `409`. A retry with the same `Idempotency-Key` returns the same trade.
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
    ///         .trades
    ///         .create(
    ///             &CreateTradesRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 r#type: CreateTradesRequestType::Close,
    ///                 amount: None,
    ///                 client_order_id: None,
    ///                 instrument_type: None,
    ///                 leverage: None,
    ///                 market: None,
    ///                 metadata: None,
    ///                 notional: None,
    ///                 order_type: None,
    ///                 price: None,
    ///                 reduce_only: None,
    ///                 side: None,
    ///                 size: None,
    ///                 slippage_bps: None,
    ///                 stop_loss: None,
    ///                 take_profit: None,
    ///                 time_in_force: None,
    ///                 trigger_price: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateTradesRequest,
        options: Option<RequestOptions>,
    ) -> Result<Trade, ApiError> {
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
                "trades",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a trade. Read it until its `status` is `completed`, `failed` or `in_review`.
    ///
    /// # Arguments
    ///
    /// * `id` - Trade ID, prefixed `tint_`.
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
    ///     client.trades.retrieve(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Trade, ApiError> {
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
                &format!("trades/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancels the order of a trade while it rests on the book, such as a `good_til_canceled` limit order. Answers with the trade still `processing`; read it with `GET /trades/:id` until its `status` is `completed` or `failed`. A part that filled before the cancel landed stays filled, and money bridged in for an order that did not fill goes back to the wallet. Calling it again sends no second cancel. Answers `409` when the trade has no order on the book.
    ///
    /// # Arguments
    ///
    /// * `id` - Trade ID, prefixed `tint_`.
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
    ///     client.trades.cancel(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Trade, ApiError> {
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
                &format!("trades/{}/cancel", id),
                None,
                None,
                options,
            )
            .await
    }
}
