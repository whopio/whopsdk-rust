pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CashbackRulesListQueryRequest {
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
    /// Field to sort by. Defaults to created_at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListCashbackRulesRequestOrder>,
    /// Sort direction. Defaults to desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListCashbackRulesRequestDirection>,
}

impl CashbackRulesListQueryRequest {
    pub fn builder() -> CashbackRulesListQueryRequestBuilder {
        <CashbackRulesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CashbackRulesListQueryRequestBuilder {
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListCashbackRulesRequestOrder>,
    direction: Option<ListCashbackRulesRequestDirection>,
}

impl CashbackRulesListQueryRequestBuilder {
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

    pub fn order(mut self, value: ListCashbackRulesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListCashbackRulesRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CashbackRulesListQueryRequest`].
    pub fn build(self) -> Result<CashbackRulesListQueryRequest, BuildError> {
        Ok(CashbackRulesListQueryRequest {
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
