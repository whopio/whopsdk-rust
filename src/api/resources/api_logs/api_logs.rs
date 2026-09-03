use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
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
    /// * `first` - Number of logs to return.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
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
    ///             &APILogsListQueryRequest {
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
                .or_insert_with(|| "2026-09-02-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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
}
