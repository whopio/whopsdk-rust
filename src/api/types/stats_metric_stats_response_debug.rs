pub use crate::prelude::*;

/// Debug information including engine and SQL.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricStatsResponseDebug {
    /// The query engine used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// Unique request identifier for debugging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// The generated SQL query (with IDs sanitized).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
}

impl MetricStatsResponseDebug {
    pub fn builder() -> MetricStatsResponseDebugBuilder {
        <MetricStatsResponseDebugBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricStatsResponseDebugBuilder {
    engine: Option<String>,
    request_id: Option<String>,
    sql: Option<String>,
}

impl MetricStatsResponseDebugBuilder {
    pub fn engine(mut self, value: impl Into<String>) -> Self {
        self.engine = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn sql(mut self, value: impl Into<String>) -> Self {
        self.sql = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MetricStatsResponseDebug`].
    pub fn build(self) -> Result<MetricStatsResponseDebug, BuildError> {
        Ok(MetricStatsResponseDebug {
            engine: self.engine,
            request_id: self.request_id,
            sql: self.sql,
        })
    }
}
