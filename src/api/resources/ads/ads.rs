use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AdsClient {
    pub http_client: HttpClient,
}

impl AdsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists the ads for an account, with stats over the requested window.
    ///
    /// # Arguments
    ///
    /// * `account_id` - The account the ads belong to. Defaults to the account-scoped key's own account.
    /// * `ad_campaign_id` - Only return ads in this ad campaign.
    /// * `ad_campaign_ids` - Only return ads in these ad campaigns (max 100). Repeat the parameter for each id (ad_campaign_ids=a&ad_campaign_ids=b).
    /// * `ad_group_id` - Only return ads in this ad group.
    /// * `ad_group_ids` - Only return ads in these ad groups (max 100). Repeat the parameter for each id (ad_group_ids=a&ad_group_ids=b).
    /// * `status` - Only return ads with this status.
    /// * `query` - Filter ads by a title or ID substring.
    /// * `order` - The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports.
    /// * `direction` - The sort direction. Defaults to desc.
    /// * `created_before` - Only return ads created before this timestamp.
    /// * `created_after` - Only return ads created after this timestamp.
    /// * `stats_from` - Start of the stats window. Defaults to all-time.
    /// * `stats_to` - End of the stats window. Defaults to now.
    /// * `time_zone` - IANA timezone (e.g. America/New_York) the stats window is interpreted in. Bare stats_from/stats_to dates resolve to day boundaries on this clock. Defaults to UTC.
    /// * `attribution_model` - Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    /// * `first` - The number of ads to return.
    /// * `after` - Cursor to fetch the page after (from page_info.end_cursor).
    /// * `last` - The number of ads to return from the end of the range.
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
    ///         .ads
    ///         .list(
    ///             &AdsListQueryRequest {
    ///                 ad_campaign_ids: vec![Some("adcamp_xxxxxxxxxxxxxx".to_string())],
    ///                 ad_group_ids: vec![Some("adgrp_xxxxxxxxxxxxxx".to_string())],
    ///                 account_id: None,
    ///                 ad_campaign_id: None,
    ///                 ad_group_id: None,
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
        request: &AdsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListAdsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "ads",
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .string("ad_campaign_id", request.ad_campaign_id.clone())
                    .string_array("ad_campaign_ids", request.ad_campaign_ids.clone())
                    .string("ad_group_id", request.ad_group_id.clone())
                    .string_array("ad_group_ids", request.ad_group_ids.clone())
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

    /// Creates an ad in an ad group.
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
    ///         .ads
    ///         .create(
    ///             &CreateAdsRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateAdsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Ad, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                "ads",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a single ad with stats over the requested window.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad ID.
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
    ///         .ads
    ///         .retrieve(
    ///             &"id".to_string(),
    ///             &AdsRetrieveQueryRequest {
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
        request: &AdsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Ad, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("ads/{}", id),
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

    /// Deletes an ad.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad ID.
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
    ///     client.ads.delete(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DeleteAdsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(Method::DELETE, &format!("ads/{}", id), None, None, options)
            .await
    }

    /// Updates an ad's editable fields.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad ID.
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
    ///         .ads
    ///         .update(
    ///             &"id".to_string(),
    ///             &UpdateAdsRequest {
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
        request: &UpdateAdsRequest,
        options: Option<RequestOptions>,
    ) -> Result<Ad, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("ads/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Copies the ad into its own ad group, or into target_ad_group_id (which must belong to the same account and be compatible with the ad). Copies keep the source ad's active/paused state.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad ID.
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
    ///         .ads
    ///         .duplicate(
    ///             &"id".to_string(),
    ///             &DuplicateAdsRequest {
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
        request: &DuplicateAdsRequest,
        options: Option<RequestOptions>,
    ) -> Result<DuplicateAdsResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ads/{}/duplicate", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Pauses an active ad.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad ID.
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
    ///     client.ads.pause(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn pause(&self, id: &str, options: Option<RequestOptions>) -> Result<Ad, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ads/{}/pause", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resumes a paused ad.
    ///
    /// # Arguments
    ///
    /// * `id` - The ad ID.
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
    ///     client.ads.unpause(&"id".to_string(), None).await;
    /// }
    /// ```
    pub async fn unpause(&self, id: &str, options: Option<RequestOptions>) -> Result<Ad, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-09-02-1".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ads/{}/unpause", id),
                None,
                None,
                options,
            )
            .await
    }
}
