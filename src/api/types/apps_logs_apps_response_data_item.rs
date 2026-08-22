pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LogsAppsResponseDataItem {
    #[serde(default)]
    pub app_build_id: String,
    #[serde(default)]
    pub app_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_time_ms: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    pub level: LogsAppsResponseDataItemLevel,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<String>,
    #[serde(default)]
    pub request_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status: Option<i64>,
    pub source: LogsAppsResponseDataItemSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wall_time_ms: Option<i64>,
}

impl LogsAppsResponseDataItem {
    pub fn builder() -> LogsAppsResponseDataItemBuilder {
        <LogsAppsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LogsAppsResponseDataItemBuilder {
    app_build_id: Option<String>,
    app_id: Option<String>,
    cpu_time_ms: Option<i64>,
    created_at: Option<DateTime<FixedOffset>>,
    level: Option<LogsAppsResponseDataItemLevel>,
    message: Option<String>,
    outcome: Option<String>,
    request_id: Option<String>,
    request_method: Option<String>,
    request_path: Option<String>,
    response_status: Option<i64>,
    source: Option<LogsAppsResponseDataItemSource>,
    stack: Option<String>,
    truncated: Option<bool>,
    wall_time_ms: Option<i64>,
}

impl LogsAppsResponseDataItemBuilder {
    pub fn app_build_id(mut self, value: impl Into<String>) -> Self {
        self.app_build_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn cpu_time_ms(mut self, value: i64) -> Self {
        self.cpu_time_ms = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn level(mut self, value: LogsAppsResponseDataItemLevel) -> Self {
        self.level = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: impl Into<String>) -> Self {
        self.outcome = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn request_method(mut self, value: impl Into<String>) -> Self {
        self.request_method = Some(value.into());
        self
    }

    pub fn request_path(mut self, value: impl Into<String>) -> Self {
        self.request_path = Some(value.into());
        self
    }

    pub fn response_status(mut self, value: i64) -> Self {
        self.response_status = Some(value);
        self
    }

    pub fn source(mut self, value: LogsAppsResponseDataItemSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn stack(mut self, value: impl Into<String>) -> Self {
        self.stack = Some(value.into());
        self
    }

    pub fn truncated(mut self, value: bool) -> Self {
        self.truncated = Some(value);
        self
    }

    pub fn wall_time_ms(mut self, value: i64) -> Self {
        self.wall_time_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LogsAppsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`app_build_id`](LogsAppsResponseDataItemBuilder::app_build_id)
    /// - [`app_id`](LogsAppsResponseDataItemBuilder::app_id)
    /// - [`created_at`](LogsAppsResponseDataItemBuilder::created_at)
    /// - [`level`](LogsAppsResponseDataItemBuilder::level)
    /// - [`message`](LogsAppsResponseDataItemBuilder::message)
    /// - [`request_id`](LogsAppsResponseDataItemBuilder::request_id)
    /// - [`source`](LogsAppsResponseDataItemBuilder::source)
    pub fn build(self) -> Result<LogsAppsResponseDataItem, BuildError> {
        Ok(LogsAppsResponseDataItem {
            app_build_id: self
                .app_build_id
                .ok_or_else(|| BuildError::missing_field("app_build_id"))?,
            app_id: self
                .app_id
                .ok_or_else(|| BuildError::missing_field("app_id"))?,
            cpu_time_ms: self.cpu_time_ms,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            level: self
                .level
                .ok_or_else(|| BuildError::missing_field("level"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            outcome: self.outcome,
            request_id: self
                .request_id
                .ok_or_else(|| BuildError::missing_field("request_id"))?,
            request_method: self.request_method,
            request_path: self.request_path,
            response_status: self.response_status,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            stack: self.stack,
            truncated: self.truncated,
            wall_time_ms: self.wall_time_ms,
        })
    }
}
