pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdCampaignsListQueryRequest {
    /// The account the campaigns belong to. Defaults to the account-scoped key's own account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only return campaigns with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAdCampaignsRequestStatus>,
    /// Filter campaigns by a title or ID substring.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// The field to sort by. Defaults to created_at. Stat columns (spend, impressions, …) rank over the stats_from/stats_to window across the whole list, not just the current page. results, cost_per_result and return_on_ad_spend rank by the same Whop pixel-attributed values the response reports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAdCampaignsRequestOrder>,
    /// The sort direction. Defaults to desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAdCampaignsRequestDirection>,
    /// Only return campaigns created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return campaigns created after this timestamp.
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
    pub attribution_model: Option<ListAdCampaignsRequestAttributionModel>,
    /// The number of campaigns to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of campaigns to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to fetch the page before (from page_info.start_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl AdCampaignsListQueryRequest {
    pub fn builder() -> AdCampaignsListQueryRequestBuilder {
        <AdCampaignsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdCampaignsListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListAdCampaignsRequestStatus>,
    query: Option<String>,
    order: Option<ListAdCampaignsRequestOrder>,
    direction: Option<ListAdCampaignsRequestDirection>,
    created_before: Option<String>,
    created_after: Option<String>,
    stats_from: Option<String>,
    stats_to: Option<String>,
    time_zone: Option<String>,
    attribution_model: Option<ListAdCampaignsRequestAttributionModel>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl AdCampaignsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListAdCampaignsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListAdCampaignsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAdCampaignsRequestDirection) -> Self {
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

    pub fn attribution_model(mut self, value: ListAdCampaignsRequestAttributionModel) -> Self {
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

    /// Consumes the builder and constructs a [`AdCampaignsListQueryRequest`].
    pub fn build(self) -> Result<AdCampaignsListQueryRequest, BuildError> {
        Ok(AdCampaignsListQueryRequest {
            account_id: self.account_id,
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
