pub use crate::prelude::*;

/// Query parameters for logs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LogsQueryRequest {
    /// Only return logs from this build.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_build_id: Option<String>,
    /// Only return console lines of this level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<LogsAppsRequestLevel>,
    /// Only return logs whose message contains this text (case-insensitive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Start of the time window as an ISO 8601 timestamp. Defaults to 7 days before created_before.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// End of the time window as an ISO 8601 timestamp. Defaults to now.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// The number of log lines to return (max 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor for fetching logs after a previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// A cursor for fetching logs before a later page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl LogsQueryRequest {
    pub fn builder() -> LogsQueryRequestBuilder {
        <LogsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LogsQueryRequestBuilder {
    app_build_id: Option<String>,
    level: Option<LogsAppsRequestLevel>,
    query: Option<String>,
    created_after: Option<DateTime<FixedOffset>>,
    created_before: Option<DateTime<FixedOffset>>,
    first: Option<i64>,
    after: Option<String>,
    before: Option<String>,
}

impl LogsQueryRequestBuilder {
    pub fn app_build_id(mut self, value: impl Into<String>) -> Self {
        self.app_build_id = Some(value.into());
        self
    }

    pub fn level(mut self, value: LogsAppsRequestLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
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

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LogsQueryRequest`].
    pub fn build(self) -> Result<LogsQueryRequest, BuildError> {
        Ok(LogsQueryRequest {
            app_build_id: self.app_build_id,
            level: self.level,
            query: self.query,
            created_after: self.created_after,
            created_before: self.created_before,
            first: self.first,
            after: self.after,
            before: self.before,
        })
    }
}
