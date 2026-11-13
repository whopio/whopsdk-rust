use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct LeadsClient {
    pub http_client: HttpClient,
}

impl LeadsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists an account's leads, newest first.
    ///
    /// Required permissions:
    /// - `lead:basic:read`
    /// - `member:email:read`
    /// - `access_pass:basic:read`
    /// - `member:basic:read`
    ///
    /// # Arguments
    ///
    /// * `after` - Returns the elements in the list that come after the specified cursor.
    /// * `before` - Returns the elements in the list that come before the specified cursor.
    /// * `first` - Returns the first _n_ elements from the list.
    /// * `last` - Returns the last _n_ elements from the list.
    /// * `created_after` - Only return leads created after this timestamp.
    /// * `created_before` - Only return leads created before this timestamp.
    /// * `product_ids` - Filter leads to only those associated with these specific product identifiers.
    /// * `account_id` - The unique identifier of the company to list leads for.
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
    ///         .leads
    ///         .list(
    ///             &LeadsListQueryRequest {
    ///                 first: Some(42),
    ///                 last: Some(42),
    ///                 created_after: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 created_before: Some(DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap()),
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 after: None,
    ///                 before: None,
    ///                 product_ids: vec![],
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &LeadsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListLeadsResponse, ApiError> {
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
                "leads",
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("before", request.before.clone())
                    .int("first", request.first.clone())
                    .int("last", request.last.clone())
                    .datetime("created_after", request.created_after.clone())
                    .datetime("created_before", request.created_before.clone())
                    .string_array("product_ids", request.product_ids.clone())
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn list_paginated(
        &self,
        request: &LeadsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("before", request.before.clone())
            .int("first", request.first.clone())
            .int("last", request.last.clone())
            .datetime("created_after", request.created_after.clone())
            .datetime("created_before", request.created_before.clone())
            .string_array("product_ids", request.product_ids.clone())
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
                            "leads",
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

    /// Records a lead: a potential customer's interest in an account or one of its products.
    ///
    /// Required permissions:
    /// - `lead:manage`
    /// - `member:email:read`
    /// - `access_pass:basic:read`
    /// - `member:basic:read`
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
    ///         .leads
    ///         .create(
    ///             &CreateLeadsRequest {
    ///                 account_id: "biz_xxxxxxxxxxxxxx".to_string(),
    ///                 metadata: None,
    ///                 product_id: None,
    ///                 referrer: None,
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Lead, ApiError> {
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
                "leads",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the details of an existing lead.
    ///
    /// Required permissions:
    /// - `lead:basic:read`
    /// - `member:email:read`
    /// - `access_pass:basic:read`
    /// - `member:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the lead to retrieve.
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
    ///         .leads
    ///         .retrieve(&"lead_xxxxxxxxxxxxx".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<Lead, ApiError> {
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
                &format!("leads/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a lead's `metadata` or `referrer`.
    ///
    /// Required permissions:
    /// - `lead:manage`
    /// - `member:email:read`
    /// - `access_pass:basic:read`
    /// - `member:basic:read`
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the lead to update, starting with 'lead_'.
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
    ///         .leads
    ///         .update(
    ///             &"lead_xxxxxxxxxxxxx".to_string(),
    ///             &UpdateLeadsRequest {
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
        request: &UpdateLeadsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Lead, ApiError> {
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
                &format!("leads/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
