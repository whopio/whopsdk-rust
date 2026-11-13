use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct AdGroupsClient {
    pub http_client: HttpClient,
}

impl AdGroupsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists ad groups for the account, newest first.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account whose ad groups to list. Defaults to the authenticated account.
    /// * `ad_campaign_id` - Filter to ad groups in this campaign.
    /// * `ad_campaign_ids` - Filter to ad groups in these campaigns (max 100). Repeat the parameter for each id (ad_campaign_ids=a&ad_campaign_ids=b).
    /// * `status` - Filter to ad groups with this status.
    /// * `query` - Filter ad groups by a title or ID substring.
    /// * `order` - The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports, and link_clicks by the same Whop pixel page views.
    /// * `direction` - The sort direction. Defaults to desc.
    /// * `created_before` - Only return ad groups created before this timestamp.
    /// * `created_after` - Only return ad groups created after this timestamp.
    /// * `stats_from` - Start of the stats window. Defaults to all-time.
    /// * `stats_to` - End of the stats window. Defaults to now.
    /// * `time_zone` - IANA timezone (e.g. America/New_York) the stats window is interpreted in. Bare stats_from/stats_to dates resolve to day boundaries on this clock. Defaults to UTC.
    /// * `attribution_model` - Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
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
    ///         .ad_groups
    ///         .list(
    ///             &AdGroupsListQueryRequest {
    ///                 ad_campaign_ids: vec![Some("adcamp_xxxxxxxxxxxxxx".to_string())],
    ///                 account_id: None,
    ///                 ad_campaign_id: None,
    ///                 status: None,
    ///                 query: None,
    ///                 order: None,
    ///                 direction: None,
    ///                 created_before: None,
    ///                 created_after: None,
    ///                 stats_from: None,
    ///                 stats_to: None,
    ///                 time_zone: None,
    ///                 attribution_model: None,
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
        request: &AdGroupsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAdGroupsResponse, ApiError> {
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
                "ad_groups",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("ad_campaign_id", request.ad_campaign_id.clone())
                    .string_array("ad_campaign_ids", request.ad_campaign_ids.clone())
                    .serialize("status", request.status.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .string("created_before", request.created_before.clone())
                    .string("created_after", request.created_after.clone())
                    .string("stats_from", request.stats_from.clone())
                    .string("stats_to", request.stats_to.clone())
                    .string("time_zone", request.time_zone.clone())
                    .serialize("attribution_model", request.attribution_model.clone())
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
        request: &AdGroupsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .string("ad_campaign_id", request.ad_campaign_id.clone())
            .string_array("ad_campaign_ids", request.ad_campaign_ids.clone())
            .serialize("status", request.status.clone())
            .structured_query("query", request.query.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .string("created_before", request.created_before.clone())
            .string("created_after", request.created_after.clone())
            .string("stats_from", request.stats_from.clone())
            .string("stats_to", request.stats_to.clone())
            .string("time_zone", request.time_zone.clone())
            .serialize("attribution_model", request.attribution_model.clone())
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
                            "ad_groups",
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

    /// Creates an ad group (ad set) in a campaign.
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
    ///         .ad_groups
    ///         .create(
    ///             &CreateAdGroupsRequest {
    ///                 ad_campaign_id: "adcamp_xxxxxxxxxxxxxx".to_string(),
    ///                 audiences: None,
    ///                 bid_type: None,
    ///                 budget_amount: None,
    ///                 budget_amount_local: None,
    ///                 budget_type: None,
    ///                 conversion_event: None,
    ///                 delivery_schedule: None,
    ///                 demographics: None,
    ///                 desired_cost_per_result: None,
    ///                 devices: None,
    ///                 ends_at: None,
    ///                 keywords: None,
    ///                 languages: None,
    ///                 regions: None,
    ///                 starts_at: None,
    ///                 status: None,
    ///                 title: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAdGroupsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdGroup, ApiError> {
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
                "ad_groups",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves an ad group, with performance stats for the window set by `stats_from` and `stats_to`.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad group ID.
    /// * `stats_from` - Start of the stats window.
    /// * `stats_to` - End of the stats window.
    /// * `time_zone` - IANA timezone the stats window is interpreted in. Defaults to UTC.
    /// * `attribution_model` - Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
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
    ///         .ad_groups
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &AdGroupsRetrieveQueryRequest {
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
        request: &AdGroupsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdGroup, ApiError> {
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
                &format!("ad_groups/{}", id),
                None,
                QueryBuilder::new()
                    .string("stats_from", request.stats_from.clone())
                    .string("stats_to", request.stats_to.clone())
                    .string("time_zone", request.time_zone.clone())
                    .serialize("attribution_model", request.attribution_model.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes an ad group and stops its delivery.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad group ID.
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
    ///     client.ad_groups.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteAdGroupsResponse, ApiError> {
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
                &format!("ad_groups/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an ad group's editable fields. Only the keys you send are changed.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad group ID.
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
    ///         .ad_groups
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateAdGroupsRequest {
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
        request: &UpdateAdGroupsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdGroup, ApiError> {
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
                &format!("ad_groups/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Starts copying an ad group and returns the copies in `duplicating` status. Poll each returned ad group until it leaves `duplicating`: it then takes the source's `active` or `paused` status, or, if the copy could not be completed, is deleted and returns 404.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad group ID.
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
    ///         .ad_groups
    ///         .duplicate(
    ///             &"id".to_string(),
    ///             &DuplicateAdGroupsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn duplicate(
        &self,
        id: &str,
        request: &DuplicateAdGroupsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DuplicateAdGroupsResponse, ApiError> {
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
                &format!("ad_groups/{}/duplicate", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pauses delivery of an ad group.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad group ID.
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
    ///     client.ad_groups.pause(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn pause(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdGroup, ApiError> {
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
                &format!("ad_groups/{}/pause", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resumes delivery of a paused ad group.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad group ID.
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
    ///     client.ad_groups.unpause(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn unpause(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdGroup, ApiError> {
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
                &format!("ad_groups/{}/unpause", id),
                None,
                None,
                options,
            )
            .await
    }
}
