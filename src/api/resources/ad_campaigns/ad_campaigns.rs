use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
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
    /// * `order` - The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports.
    /// * `direction` - The sort direction. Defaults to desc.
    /// * `created_before` - Only return campaigns created before this timestamp.
    /// * `created_after` - Only return campaigns created after this timestamp.
    /// * `stats_from` - Start of the stats window. Defaults to all-time.
    /// * `stats_to` - End of the stats window. Defaults to now.
    /// * `time_zone` - IANA timezone (e.g. America/New_York) the stats window is interpreted in. Bare stats_from/stats_to dates resolve to day boundaries on this clock. Defaults to UTC.
    /// * `attribution_model` - Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    /// * `first` - The number of campaigns to return.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - The number of campaigns to return from the end of the range.
    /// * `before` - Cursor to fetch the page before (from page_info.start_cursor).
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "ad_campaigns",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
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

    /// Creates an ad campaign for an account.
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
    ///                 objective: CreateAdCampaignsRequestObjective::Awareness,
    ///                 platform: CreateAdCampaignsRequestPlatform::Meta,
    ///                 title: "Now hiring mobile detailers — Austin".to_string(),
    ///                 account_id: None,
    ///                 bid_type: None,
    ///                 budget_amount: None,
    ///                 budget_optimization: None,
    ///                 budget_type: None,
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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

    /// Deletes an ad campaign and archives it on the ad platform (cascades to ad groups and ads).
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("ad_campaigns/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an ad campaign's editable fields (title, budget, schedule, bid strategy, special ad categories, and, before launch, budget optimization), and launches a draft campaign by setting status to active. Objective, budget type and desired cost per result are fixed at creation and cannot be changed.
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("ad_campaigns/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ad_campaigns/{}/pause", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retries billing for an ad campaign whose payment previously failed.
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
    ///         .retry_payment(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn retry_payment(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AdCampaign, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ad_campaigns/{}/retry_payment", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resumes a paused ad campaign.
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
                .or_insert_with(|| "2026-08-21-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ad_campaigns/{}/unpause", id),
                None,
                None,
                options,
            )
            .await
    }
}
