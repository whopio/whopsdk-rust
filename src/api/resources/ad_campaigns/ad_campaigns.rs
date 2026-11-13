use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use crate::{AsyncPaginator, PaginationResult};
use reqwest::Method;

pub struct AdCampaignsClient {
    pub http_client: HttpClient,
}

impl AdCampaignsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the ad campaigns for an account, with stats over the requested window.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account the campaigns belong to. Defaults to the account-scoped key's own account.
    /// * `status` - Only return campaigns with this status.
    /// * `query` - Filter campaigns by a title or ID substring.
    /// * `order` - The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports, and link_clicks by the same Whop pixel page views.
    /// * `direction` - The sort direction. Defaults to desc.
    /// * `campaign_type` - Only return campaigns of this type.
    /// * `created_before` - Only return campaigns created before this timestamp.
    /// * `created_after` - Only return campaigns created after this timestamp.
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
    ///         .ad_campaigns
    ///         .list(
    ///             &AdCampaignsListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        request: &AdCampaignsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAdCampaignsResponse, ApiError> {
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
                "ad_campaigns",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("status", request.status.clone())
                    .structured_query("query", request.query.clone())
                    .serialize("order", request.order.clone())
                    .serialize("direction", request.direction.clone())
                    .serialize("campaign_type", request.campaign_type.clone())
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
        request: &AdCampaignsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AsyncPaginator<serde_json::Value>, ApiError> {
        let http_client = std::sync::Arc::new(self.http_client.clone());
        let base_query_params = QueryBuilder::new()
            .string("account_id", request.account_id.clone())
            .serialize("status", request.status.clone())
            .structured_query("query", request.query.clone())
            .serialize("order", request.order.clone())
            .serialize("direction", request.direction.clone())
            .serialize("campaign_type", request.campaign_type.clone())
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
                            "ad_campaigns",
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

    /// Creates an ad campaign in `draft` status for an account. Nothing runs until you launch it by setting `status` to `active` with `PATCH /ad_campaigns/:id`.
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
    ///         .ad_campaigns
    ///         .create(
    ///             &CreateAdCampaignsRequest {
    ///                 objective: CreateAdCampaignsRequestObjective::Traffic,
    ///                 title: "Now hiring mobile detailers — Austin".to_string(),
    ///                 account_id: None,
    ///                 budget_amount: None,
    ///                 budget_amount_local: None,
    ///                 budget_optimization: None,
    ///                 budget_type: None,
    ///                 campaign_type: None,
    ///                 desired_cost_per_result: None,
    ///                 ends_at: None,
    ///                 special_ad_categories: None,
    ///                 starts_at: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAdCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdCampaign, ApiError> {
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
                "ad_campaigns",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a single ad campaign with stats over the requested window.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad campaign ID.
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
    ///         .ad_campaigns
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &AdCampaignsRetrieveQueryRequest {
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
        request: &AdCampaignsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdCampaign, ApiError> {
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
                &format!("ad_campaigns/{}", id),
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

    /// Deletes an ad campaign with its ad groups and ads, and stops their delivery.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad campaign ID.
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
    ///     client.ad_campaigns.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteAdCampaignsResponse, ApiError> {
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
                &format!("ad_campaigns/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an ad campaign's settings, or launches a draft campaign by setting `status` to `active`. The objective is fixed at creation and cannot be changed.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad campaign ID.
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
    ///         .ad_campaigns
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateAdCampaignsRequest {
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
        request: &UpdateAdCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdCampaign, ApiError> {
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
                &format!("ad_campaigns/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Attributes the whole campaign and its daily budget to the calling user as its runner. One runner holds a campaign at a time. Takes the runner's Whop session, or a user API key with `ad_campaign:update`. The caller also needs active advertiser-supplier program access. Everyone else gets a 404. Returns 403 until the caller has verified their identity and added a phone number and a social account. Returns 403 when the claim would take the daily budget the caller holds past the limit for their score. Returns 409 when another user holds the campaign. Changes nothing when the caller already holds it.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad campaign ID.
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
    ///     client.ad_campaigns.claim(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn claim(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdCampaign, ApiError> {
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
                &format!("ad_campaigns/{}/claim", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates copies of the campaign in `duplicating` status and returns them; each copy transitions to `draft` once duplication completes. Poll each returned campaign until it leaves `duplicating` — a copy that could not be completed is deleted and returns 404.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad campaign ID.
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
    ///         .ad_campaigns
    ///         .duplicate(
    ///             &"id".to_string(),
    ///             &DuplicateAdCampaignsRequest {
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
        request: &DuplicateAdCampaignsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DuplicateAdCampaignsResponse, ApiError> {
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
                &format!("ad_campaigns/{}/duplicate", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pauses an active ad campaign.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad campaign ID.
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
    ///     client.ad_campaigns.pause(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn pause(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdCampaign, ApiError> {
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
                &format!("ad_campaigns/{}/pause", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resumes a paused ad campaign. Requires an ads payment method on the account.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad campaign ID.
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
    ///     client.ad_campaigns.unpause(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn unpause(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdCampaign, ApiError> {
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
                &format!("ad_campaigns/{}/unpause", id),
                None,
                None,
                options,
            )
            .await
    }
}
