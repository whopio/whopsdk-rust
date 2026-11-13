use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct ApiLogsClient {
    pub http_client: HttpClient,
}

impl ApiLogsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the requests served by Whop's API with the account's API keys, newest first — every surface (GraphQL, REST, and native /api/v1), reads and failed requests included.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account (biz_*) whose API logs to list. Defaults to the authenticated account.
    /// * `created_after` - Only return requests served at or after this ISO 8601 timestamp. Defaults to 7 days before created_before, or 7 days ago.
    /// * `created_before` - Only return requests served before this ISO 8601 timestamp.
    /// * `operation_name` - Only return requests for this operation, matched exactly against the operation_name shown on each log row (for example api/v1/products#create).
    /// * `http_method` - Only return requests made with this HTTP method.
    /// * `status` - Only return requests that finished with this status.
    /// * `api_key_id` - Only return requests made with this API key (apik_…).
    /// * `min_duration_ms` - Only return requests that took at least this many milliseconds.
    /// * `max_duration_ms` - Only return requests that took at most this many milliseconds.
    /// * `first` - Number of results to return from the start of the range.
    /// * `after` - Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
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
    ///         .api_logs
    ///         .list(
    ///             &ApiLogsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &ApiLogsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListApiLogsResponse, ApiError> {
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
                "api_logs",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("created_after", request.created_after.clone())
                    .string("created_before", request.created_before.clone())
                    .string("operation_name", request.operation_name.clone())
                    .serialize("http_method", request.http_method.clone())
                    .serialize("status", request.status.clone())
                    .string("api_key_id", request.api_key_id.clone())
                    .int("min_duration_ms", request.min_duration_ms.clone())
                    .int("max_duration_ms", request.max_duration_ms.clone())
                    .int("first", request.first.clone())
                    .string("after", request.after.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &ApiLogsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .string("created_after", request.created_after.clone())
            .string("created_before", request.created_before.clone())
            .string("operation_name", request.operation_name.clone())
            .serialize("http_method", request.http_method.clone())
            .serialize("status", request.status.clone())
            .string("api_key_id", request.api_key_id.clone())
            .int("min_duration_ms", request.min_duration_ms.clone())
            .int("max_duration_ms", request.max_duration_ms.clone())
            .int("first", request.first.clone())
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
                            "api_logs",
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
