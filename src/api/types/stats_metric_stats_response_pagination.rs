pub use crate::prelude::*;

/// Pagination information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MetricStatsResponsePagination {
    /// Cursor for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl MetricStatsResponsePagination {
    pub fn builder() -> MetricStatsResponsePaginationBuilder {
        <MetricStatsResponsePaginationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MetricStatsResponsePaginationBuilder {
    next_cursor: Option<String>,
}

impl MetricStatsResponsePaginationBuilder {
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MetricStatsResponsePagination`].
    pub fn build(self) -> Result<MetricStatsResponsePagination, BuildError> {
        Ok(MetricStatsResponsePagination {
            next_cursor: self.next_cursor,
        })
    }
}
