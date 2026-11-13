use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct CandlesClient {
    pub http_client: HttpClient,
}

impl CandlesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns price candles for a tokenized stock, oldest first.
    ///
    /// - **Intervals:** `1h`, `2h`, `4h`, `6h` and `12h` candles cover the last 30 days. `1d` and `1w` candles cover the full history.
    /// - **Weeks** start on Monday at 00:00 UTC.
    ///
    /// This endpoint is public: no API key is needed, and each IP address can make up to 60 requests a minute.
    ///
    /// # Arguments
    ///
    /// * `market` - The tokenized stock's ticker, such as `NVDA`. This is the `id` of a spot market from List Markets.
    /// * `interval` - How much time each candle covers.
    /// * `from` - Return candles that start at or after this time, as an ISO 8601 timestamp.
    /// * `to` - Return candles that start before this time, as an ISO 8601 timestamp. Defaults to now.
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
    ///         .candles
    ///         .list(
    ///             &TradesCandlesListQueryRequest {
    ///                 market: "market".to_string(),
    ///                 interval: ListCandlesRequestInterval::OneH,
    ///                 from: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///                 to: None,
    ///                 first: None,
    ///                 after: None,
    ///                 last: None,
    ///                 before: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &TradesCandlesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCandlesResponse, ApiError> {
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
                "trades/candles",
                None,
                QueryBuilder::new()
                    .string("market", request.market.clone())
                    .serialize("interval", Some(request.interval.clone()))
                    .datetime("from", request.from.clone())
                    .datetime("to", request.to.clone())
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
        request: &TradesCandlesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("market", request.market.clone())
            .serialize("interval", Some(request.interval.clone()))
            .datetime("from", request.from.clone())
            .datetime("to", request.to.clone())
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
                            "trades/candles",
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
}
