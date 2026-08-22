pub use crate::prelude::*;

/// Result from a stats query (raw, metric, or SQL).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MetricStatsResponse {
    /// Column names in the order they appear in each data row.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// Array of data rows, where each row is an array of values matching the columns order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// Debug information including engine and SQL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<MetricStatsResponseDebug>,
    /// The node path that was queried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    /// Pagination information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<MetricStatsResponsePagination>,
    /// The typename of this object
    pub typename: MetricStatsResponseTypename,
}

impl MetricStatsResponse {
    pub fn builder() -> MetricStatsResponseBuilder {
        <MetricStatsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricStatsResponseBuilder {
    columns: Option<Vec<String>>,
    data: Option<Vec<HashMap<String, serde_json::Value>>>,
    debug: Option<MetricStatsResponseDebug>,
    node: Option<String>,
    pagination: Option<MetricStatsResponsePagination>,
    typename: Option<MetricStatsResponseTypename>,
}

impl MetricStatsResponseBuilder {
    pub fn columns(mut self, value: Vec<String>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn data(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn debug(mut self, value: MetricStatsResponseDebug) -> Self {
        self.debug = Some(value);
        self
    }

    pub fn node(mut self, value: impl Into<String>) -> Self {
        self.node = Some(value.into());
        self
    }

    pub fn pagination(mut self, value: MetricStatsResponsePagination) -> Self {
        self.pagination = Some(value);
        self
    }

    pub fn typename(mut self, value: MetricStatsResponseTypename) -> Self {
        self.typename = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MetricStatsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`typename`](MetricStatsResponseBuilder::typename)
    pub fn build(self) -> Result<MetricStatsResponse, BuildError> {
        Ok(MetricStatsResponse {
            columns: self.columns,
            data: self.data,
            debug: self.debug,
            node: self.node,
            pagination: self.pagination,
            typename: self
                .typename
                .ok_or_else(|| BuildError::missing_field("typename"))?,
        })
    }
}
