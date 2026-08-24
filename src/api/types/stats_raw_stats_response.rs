pub use crate::prelude::*;

/// Result from a stats query (raw, metric, or SQL).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RawStatsResponse {
    /// Column names in the order they appear in each data row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// Array of data rows, where each row is an array of values matching the columns order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// Debug information including engine and SQL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<RawStatsResponseDebug>,
    /// The node path that was queried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    /// Pagination information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<RawStatsResponsePagination>,
}

impl RawStatsResponse {
    pub fn builder() -> RawStatsResponseBuilder {
        <RawStatsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RawStatsResponseBuilder {
    columns: Option<Vec<String>>,
    data: Option<Vec<HashMap<String, serde_json::Value>>>,
    debug: Option<RawStatsResponseDebug>,
    node: Option<String>,
    pagination: Option<RawStatsResponsePagination>,
}

impl RawStatsResponseBuilder {
    pub fn columns(mut self, value: Vec<String>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn debug(mut self, value: RawStatsResponseDebug) -> Self {
        self.debug = Some(value);
        self
    }

    pub fn node(mut self, value: impl Into<String>) -> Self {
        self.node = Some(value.into());
        self
    }

    pub fn pagination(mut self, value: RawStatsResponsePagination) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RawStatsResponse`].
    pub fn build(self) -> Result<RawStatsResponse, BuildError> {
        Ok(RawStatsResponse {
            columns: self.columns,
            data: self.data,
            debug: self.debug,
            node: self.node,
            pagination: self.pagination,
        })
    }
}
