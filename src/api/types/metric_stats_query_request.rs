pub use crate::prelude::*;

/// Query parameters for metricStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MetricStatsQueryRequest {
    /// Metric resource using : as separator (e.g., 'receipts:gross_revenue', 'members:new_users').
    #[serde(default)]
    pub resource: String,
    /// Time granularity (daily, weekly, monthly).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<String>,
    /// Columns to break down the metric by.
    #[serde(default)]
    pub breakdowns: Vec<Option<String>>,
    /// Key-value pairs to filter the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
    /// IANA timezone for period bucketing (e.g. 'America/New_York'). Defaults to UTC. Only applies to ClickHouse metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// Start of time range (unix timestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub from: Option<DateTime<FixedOffset>>,
    /// End of time range (unix timestamp).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub to: Option<DateTime<FixedOffset>>,
    /// Scope query to a specific user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Scope query to a specific company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl MetricStatsQueryRequest {
    pub fn builder() -> MetricStatsQueryRequestBuilder {
        <MetricStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricStatsQueryRequestBuilder {
    resource: Option<String>,
    granularity: Option<String>,
    breakdowns: Option<Vec<Option<String>>>,
    filters: Option<HashMap<String, serde_json::Value>>,
    time_zone: Option<String>,
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    user_id: Option<String>,
    account_id: Option<String>,
}

impl MetricStatsQueryRequestBuilder {
    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = Some(value.into());
        self
    }

    pub fn granularity(mut self, value: impl Into<String>) -> Self {
        self.granularity = Some(value.into());
        self
    }

    pub fn breakdowns(mut self, value: Vec<Option<String>>) -> Self {
        self.breakdowns = Some(value);
        self
    }

    pub fn filters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn time_zone(mut self, value: impl Into<String>) -> Self {
        self.time_zone = Some(value.into());
        self
    }

    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MetricStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource`](MetricStatsQueryRequestBuilder::resource)
    /// - [`breakdowns`](MetricStatsQueryRequestBuilder::breakdowns)
    pub fn build(self) -> Result<MetricStatsQueryRequest, BuildError> {
        Ok(MetricStatsQueryRequest {
            resource: self
                .resource
                .ok_or_else(|| BuildError::missing_field("resource"))?,
            granularity: self.granularity,
            breakdowns: self
                .breakdowns
                .ok_or_else(|| BuildError::missing_field("breakdowns"))?,
            filters: self.filters,
            time_zone: self.time_zone,
            from: self.from,
            to: self.to,
            user_id: self.user_id,
            account_id: self.account_id,
        })
    }
}
