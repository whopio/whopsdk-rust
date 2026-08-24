pub use crate::prelude::*;

/// Pagination information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RawStatsResponsePagination {
    /// Cursor for the next page of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl RawStatsResponsePagination {
    pub fn builder() -> RawStatsResponsePaginationBuilder {
        <RawStatsResponsePaginationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RawStatsResponsePaginationBuilder {
    next_cursor: Option<String>,
}

impl RawStatsResponsePaginationBuilder {
    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RawStatsResponsePagination`].
    pub fn build(self) -> Result<RawStatsResponsePagination, BuildError> {
        Ok(RawStatsResponsePagination {
            next_cursor: self.next_cursor,
        })
    }
}
