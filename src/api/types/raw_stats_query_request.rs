pub use crate::prelude::*;

/// Query parameters for rawStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RawStatsQueryRequest {
    /// Resource path using : as separator (e.g., 'members', 'payments:membership').
    #[serde(default)]
    pub resource: String,
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
    /// Number of records to return (max 10000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Pagination cursor for next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Column to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<Direction>,
    /// Scope query to a specific company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// Scope query to a specific user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl RawStatsQueryRequest {
    pub fn builder() -> RawStatsQueryRequestBuilder {
        <RawStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RawStatsQueryRequestBuilder {
    resource: Option<String>,
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    limit: Option<i64>,
    cursor: Option<String>,
    sort: Option<String>,
    sort_direction: Option<Direction>,
    company_id: Option<String>,
    user_id: Option<String>,
}

impl RawStatsQueryRequestBuilder {
    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = Some(value.into());
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

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn sort(mut self, value: impl Into<String>) -> Self {
        self.sort = Some(value.into());
        self
    }

    pub fn sort_direction(mut self, value: Direction) -> Self {
        self.sort_direction = Some(value);
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RawStatsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource`](RawStatsQueryRequestBuilder::resource)
    pub fn build(self) -> Result<RawStatsQueryRequest, BuildError> {
        Ok(RawStatsQueryRequest {
            resource: self
                .resource
                .ok_or_else(|| BuildError::missing_field("resource"))?,
            from: self.from,
            to: self.to,
            limit: self.limit,
            cursor: self.cursor,
            sort: self.sort,
            sort_direction: self.sort_direction,
            company_id: self.company_id,
            user_id: self.user_id,
        })
    }
}
