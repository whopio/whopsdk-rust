use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
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
    /// * `order` - The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports.
    /// * `direction` - The sort direction. Defaults to desc.
    /// * `created_before` - Only return ad groups created before this timestamp.
    /// * `created_after` - Only return ad groups created after this timestamp.
    /// * `stats_from` - Start of the stats window. Defaults to all-time.
    /// * `stats_to` - End of the stats window. Defaults to now.
    /// * `time_zone` - IANA timezone (e.g. America/New_York) the stats window is interpreted in. Bare stats_from/stats_to dates resolve to day boundaries on this clock. Defaults to UTC.
    /// * `attribution_model` - Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    /// * `first` - The number of ad groups to return.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - The number of ad groups to return from the end of the range.
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
        self.http_client
            .execute_request(
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
    ///                 budget_type: None,
    ///                 conversion_event: None,
    ///                 conversion_location: None,
    ///                 demographics: None,
    ///                 desired_cost_per_result: None,
    ///                 detailed_targeting: None,
    ///                 devices: None,
    ///                 dynamic_creative: None,
    ///                 ends_at: None,
    ///                 frequency_cap: None,
    ///                 languages: None,
    ///                 message_apps: None,
    ///                 minimum_daily_spend: None,
    ///                 optimization_goal: None,
    ///                 placements: None,
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
        self.http_client
            .execute_request(
                Method::POST,
                "ad_groups",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Estimates how many people a draft targeting spec can reach, before an ad group is created. The body takes the same targeting fields as creating an ad group — `regions`, `demographics`, `detailed_targeting`, `audiences`, `languages`, and `devices` — and nothing is persisted.
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
    ///         .estimate_reach(
    ///             &EstimateReachAdGroupsRequest {
    ///                 platform: EstimateReachAdGroupsRequestPlatform::Meta,
    ///                 account_id: None,
    ///                 audiences: None,
    ///                 demographics: None,
    ///                 detailed_targeting: None,
    ///                 devices: None,
    ///                 languages: None,
    ///                 regions: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn estimate_reach(
        &self,
        request: &EstimateReachAdGroupsRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReachEstimate, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "ad_groups/estimate_reach",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Searches the ad platform's targeting taxonomy for options to target an ad group with. Each result comes back in the exact shape the ad-group body accepts for its `type`, so it can be used in `detailed_targeting`, `regions`, or `languages` as-is. A blank `query` browses the small fixed lists (behaviors, browse demographic categories, languages); interests, work employers, job titles, schools, majors, and locations need a search term.
    ///
    /// # Arguments
    ///
    /// * `account_id` - Account to search on behalf of. Defaults to the authenticated account.
    /// * `platform` - The ad network whose targeting taxonomy to search.
    /// * `query` - The search term. Blank browses the fixed lists; interests, work employers, job titles, schools, majors, and locations return nothing without one.
    /// * `types` - Kinds of targeting options to search. Defaults to all of them.
    /// * `location_types` - Narrow location results to these kinds of places. Only applies when `types` includes `locations`.
    /// * `country` - Narrow location results to one country, as an ISO 3166-1 code such as `US`. Only applies when `types` includes `locations`.
    /// * `limit` - Maximum number of results per requested type.
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
    ///         .search_targeting_options(
    ///             &SearchTargetingOptionsQueryRequest {
    ///                 platform: SearchTargetingOptionsAdGroupsRequestPlatform::Meta,
    ///                 account_id: None,
    ///                 query: None,
    ///                 types: vec![],
    ///                 location_types: vec![],
    ///                 country: None,
    ///                 limit: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_targeting_options(
        &self,
        request: &SearchTargetingOptionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchTargetingOptionsAdGroupsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "ad_groups/targeting_options",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("platform", Some(request.platform.clone()))
                    .structured_query("query", request.query.clone())
                    .serialize_array("types", request.types.clone())
                    .serialize_array("location_types", request.location_types.clone())
                    .string("country", request.country.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a single ad group.
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
        self.http_client
            .execute_request(
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

    /// Deletes an ad group.
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
        self.http_client
            .execute_request(
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
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("ad_groups/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates copies of the ad group in `duplicating` status and returns them — into its own campaign, or into target_ad_campaign_id (which must belong to the same account and be compatible with the ad group's targeting and goals); each copy transitions to its final status (matching the source's active/paused state) once duplication completes. Poll each returned ad group until it leaves `duplicating` — a copy that could not be completed is deleted and returns 404.
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
        self.http_client
            .execute_request(
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
        self.http_client
            .execute_request(
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
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ad_groups/{}/unpause", id),
                None,
                None,
                options,
            )
            .await
    }
}
