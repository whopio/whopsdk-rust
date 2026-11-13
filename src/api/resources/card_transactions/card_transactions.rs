use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct CardTransactionsClient {
    pub http_client: HttpClient,
}

impl CardTransactionsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the card transactions of an account or a user, newest first. Covers every card the owner has ever had, including canceled cards and spend that predates a re-application, and team members only see transactions on the cards assigned to them. Pass `transaction_ids` to fetch specific transactions instead of paging for them.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account whose card transactions to list, prefixed `biz_`. Provide this or `user_id`. Defaults to the credential's account.
    /// * `user_id` - The user whose personal card transactions to list, prefixed `user_`. Provide this or `account_id`. Only that user can read them.
    /// * `transaction_ids` - Return only these card transactions, each prefixed `citx_`. Repeat the parameter, or pass one comma-separated value.
    /// * `card_id` - Return only transactions charged to these cards, each prefixed `icrd_`.
    /// * `cardholder_id` - Return only transactions on cards assigned to these users, each prefixed `user_`.
    /// * `status` - Return only transactions with this status.
    /// * `created_after` - Return only transactions authorized at or after this ISO 8601 timestamp.
    /// * `created_before` - Return only transactions authorized at or before this ISO 8601 timestamp.
    /// * `order` - The field to sort by. Defaults to `created_at`.
    /// * `direction` - The sort direction. Defaults to `desc`.
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
    ///         .card_transactions
    ///         .list(
    ///             &CardTransactionsListQueryRequest {
    ///                 transaction_ids: vec![Some("citx_xxxxxxxxxxxxxx".to_string())],
    ///                 card_id: vec![Some("icrd_xxxxxxxxxxxxxx".to_string())],
    ///                 cardholder_id: vec![Some("user_xxxxxxxxxxxxxx".to_string())],
    ///                 account_id: None,
    ///                 user_id: None,
    ///                 status: None,
    ///                 created_after: None,
    ///                 created_before: None,
    ///                 order: None,
    ///                 direction: None,
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
        request: &CardTransactionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListCardTransactionsResponse, ApiError> {
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
                "card_transactions",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string_array("transaction_ids", request.transaction_ids.clone())
                    .string_array("card_id", request.card_id.clone())
                    .string_array("cardholder_id", request.cardholder_id.clone())
                    .serialize("status", request.status.clone())
                    .string("created_after", request.created_after.clone())
                    .string("created_before", request.created_before.clone())
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
        request: &CardTransactionsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .string("user_id", request.user_id.clone())
            .string_array("transaction_ids", request.transaction_ids.clone())
            .string_array("card_id", request.card_id.clone())
            .string_array("cardholder_id", request.cardholder_id.clone())
            .serialize("status", request.status.clone())
            .string("created_after", request.created_after.clone())
            .string("created_before", request.created_before.clone())
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
                            "card_transactions",
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

    /// Retrieves a single card transaction from any card the owner has ever had, including canceled cards. Team members can retrieve only transactions on the cards assigned to them.
    ///
    /// # Arguments
    ///
    /// * `id` - The card transaction ID, prefixed `citx_`.
    /// * `account_id` - The account that owns the transaction, prefixed `biz_`. Provide this or `user_id`. Defaults to the credential's account.
    /// * `user_id` - The user that owns the transaction, prefixed `user_`. Provide this or `account_id`. Only that user can read it.
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
    ///         .card_transactions
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &CardTransactionsRetrieveQueryRequest {
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
        request: &CardTransactionsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CardTransaction, ApiError> {
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
                &format!("card_transactions/{}", id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("user_id", request.user_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
