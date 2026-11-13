use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct SupportChannelsClient {
    pub http_client: HttpClient,
}

impl SupportChannelsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists support channels between an account's team and its customers, most recently active first by default. Pass `open=true` to find channels awaiting a support response.
    ///
    /// Required permissions:
    /// - `support_chat:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `open` - Whether to filter by open or resolved support channels. Set to true to only return channels awaiting a response, or false for resolved channels.
    /// * `customer_user_id` - Only return support channels opened by this customer.
    /// * `customer_user_ids` - Only return support channels opened by any of these customers. Maximum 100 IDs per request.
    /// * `participated` - Set to true to only return channels the authenticated user has joined as a participant. Requires a user.
    /// * `include_last_message` - Set to true to return `last_message` and `unread` on each channel. Both are `null` otherwise, which keeps large lists fast.
    /// * `include_participated` - Set to true to return `participated` on each channel. It is `null` otherwise, which keeps large lists fast.
    /// * `account_id` - The unique identifier of the company to list support channels for. Includes channels of child companies. When omitted, returns support channels across all companies the user has access to.
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
    ///         .support_channels
    ///         .list(
    ///             &SupportChannelsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 account_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 after: None,
    ///                 before: None,
    ///                 view: None,
    ///                 open: None,
    ///                 customer_user_id: None,
    ///                 customer_user_ids: vec![],
    ///                 participated: None,
    ///                 include_last_message: None,
    ///                 include_participated: None,
    ///                 direction: None,
    ///                 order: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &SupportChannelsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListSupportChannelsResponse, ApiError> {
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
                "support_channels",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .serialize("view", request.view.clone())
                    .bool("open", request.open.clone())
                    .string("customer_user_id", request.customer_user_id.clone())
                    .string_array("customer_user_ids", request.customer_user_ids.clone())
                    .bool("participated", request.participated.clone())
                    .bool("include_last_message", request.include_last_message.clone())
                    .bool("include_participated", request.include_participated.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize("order", request.order.clone())
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &SupportChannelsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("before", request.before.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .serialize("view", request.view.clone())
            .bool("open", request.open.clone())
            .string("customer_user_id", request.customer_user_id.clone())
            .string_array("customer_user_ids", request.customer_user_ids.clone())
            .bool("participated", request.participated.clone())
            .bool("include_last_message", request.include_last_message.clone())
            .bool("include_participated", request.include_participated.clone())
            .serialize("direction", request.direction.clone())
            .serialize("order", request.order.clone())
            .string("account_id", request.account_id.clone())
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
                            "support_channels",
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

    /// Opens a support channel between an account's team and a customer. Returns the existing channel if that customer already has one.
    ///
    /// Required permissions:
    /// - `support_chat:create`
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
    ///         .support_channels
    ///         .create(
    ///             &CreateSupportChannelsRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 user_id: "user_xxxxxxxxxxxxx".to_string(),
    ///                 custom_name: None,
    ///                 notifications_enabled: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateSupportChannelsRequest,
        options: Option<RequestOptions>,
    ) -> Result<SupportChannel, ApiError> {
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
                "support_channels",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing support channel.
    ///
    /// Required permissions:
    /// - `support_chat:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the support channel to retrieve.
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
    ///         .support_channels
    ///         .retrieve(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SupportChannel, ApiError> {
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
                &format!("support_channels/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
