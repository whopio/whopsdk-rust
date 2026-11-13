pub use crate::prelude::*;

/// Query parameters for search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchQueryRequest {
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The account name to search for.
    #[serde(default)]
    pub query: String,
    /// Sort field. Without it, support searches return the best match first and other searches return the newest account first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<SearchAccountsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<SearchAccountsRequestDirection>,
}

impl SearchQueryRequest {
    pub fn builder() -> SearchQueryRequestBuilder {
        <SearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    query: Option<String>,
    order: Option<SearchAccountsRequestOrder>,
    direction: Option<SearchAccountsRequestDirection>,
}

impl SearchQueryRequestBuilder {
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

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn order(mut self, value: SearchAccountsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: SearchAccountsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](SearchQueryRequestBuilder::query)
    pub fn build(self) -> Result<SearchQueryRequest, BuildError> {
        Ok(SearchQueryRequest {
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            order: self.order,
            direction: self.direction,
        })
    }
}
