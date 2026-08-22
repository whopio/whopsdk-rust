pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupsListQueryRequest {
    /// Account whose ad groups to list. Defaults to the authenticated account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Filter to ad groups in this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_campaign_id: Option<String>,
    /// Filter to ad groups in these campaigns (max 100). Repeat the parameter for each id (ad_campaign_ids=a&ad_campaign_ids=b).
    #[serde(default)]
    pub ad_campaign_ids: Vec<Option<String>>,
    /// Filter to ad groups with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAdGroupsRequestStatus>,
    /// Filter ad groups by a title or ID substring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAdGroupsRequestOrder>,
    /// The sort direction. Defaults to desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAdGroupsRequestDirection>,
    /// Only return ad groups created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return ad groups created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Start of the stats window. Defaults to all-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_from: Option<String>,
    /// End of the stats window. Defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_to: Option<String>,
    /// IANA timezone (e.g. America/New_York) the stats window is interpreted in. Bare stats_from/stats_to dates resolve to day boundaries on this clock. Defaults to UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Attribution model the conversion stats count under (defaults to last_touch). Under both models a journey with any whop ad touch attributes to whop; the model picks which whop touch credits the entity and which non-whop source wins otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution_model: Option<ListAdGroupsRequestAttributionModel>,
    /// The number of ad groups to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of ad groups to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to fetch the page before (from page_info.start_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl AdGroupsListQueryRequest {
    pub fn builder() -> AdGroupsListQueryRequestBuilder {
        <AdGroupsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupsListQueryRequestBuilder {
    account_id: Option<String>,
    ad_campaign_id: Option<String>,
    ad_campaign_ids: Option<Vec<Option<String>>>,
    status: Option<ListAdGroupsRequestStatus>,
    query: Option<String>,
    order: Option<ListAdGroupsRequestOrder>,
    direction: Option<ListAdGroupsRequestDirection>,
    created_before: Option<String>,
    created_after: Option<String>,
    stats_from: Option<String>,
    stats_to: Option<String>,
    time_zone: Option<String>,
    attribution_model: Option<ListAdGroupsRequestAttributionModel>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl AdGroupsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn ad_campaign_id(mut self, value: impl Into<String>) -> Self {
        self.ad_campaign_id = Some(value.into());
        self
    }

    pub fn ad_campaign_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.ad_campaign_ids = Some(value);
        self
    }

    pub fn status(mut self, value: ListAdGroupsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListAdGroupsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAdGroupsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn stats_from(mut self, value: impl Into<String>) -> Self {
        self.stats_from = Some(value.into());
        self
    }

    pub fn stats_to(mut self, value: impl Into<String>) -> Self {
        self.stats_to = Some(value.into());
        self
    }

    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    pub fn attribution_model(mut self, value: ListAdGroupsRequestAttributionModel) -> Self {
        self.attribution_model = Some(value);
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdGroupsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ad_campaign_ids`](AdGroupsListQueryRequestBuilder::ad_campaign_ids)
    pub fn build(self) -> Result<AdGroupsListQueryRequest, BuildError> {
        Ok(AdGroupsListQueryRequest {
            account_id: self.account_id,
            ad_campaign_id: self.ad_campaign_id,
            ad_campaign_ids: self
                .ad_campaign_ids
                .ok_or_else(|| BuildError::missing_field("ad_campaign_ids"))?,
            status: self.status,
            query: self.query,
            order: self.order,
            direction: self.direction,
            created_before: self.created_before,
            created_after: self.created_after,
            stats_from: self.stats_from,
            stats_to: self.stats_to,
            time_zone: self.time_zone,
            attribution_model: self.attribution_model,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
