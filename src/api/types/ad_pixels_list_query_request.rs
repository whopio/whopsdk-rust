pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdPixelsListQueryRequest {
    /// Only return the pixels for this business, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAdPixelsRequestOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAdPixelsRequestDirection>,
}

impl AdPixelsListQueryRequest {
    pub fn builder() -> AdPixelsListQueryRequestBuilder {
        <AdPixelsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdPixelsListQueryRequestBuilder {
    account_id: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListAdPixelsRequestOrder>,
    direction: Option<ListAdPixelsRequestDirection>,
}

impl AdPixelsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
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

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListAdPixelsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAdPixelsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdPixelsListQueryRequest`].
    pub fn build(self) -> Result<AdPixelsListQueryRequest, BuildError> {
        Ok(AdPixelsListQueryRequest {
            account_id: self.account_id,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
