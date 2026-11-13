pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TradesListQueryRequest {
    /// Only return trades for this account or user, prefixed `biz_` or `user_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only return trades with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTradesRequestStatus>,
    /// Only return trades of this type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListTradesRequestType>,
    /// Field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListTradesRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListTradesRequestDirection>,
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
}

impl TradesListQueryRequest {
    pub fn builder() -> TradesListQueryRequestBuilder {
        <TradesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradesListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListTradesRequestStatus>,
    r#type: Option<ListTradesRequestType>,
    order: Option<ListTradesRequestOrder>,
    direction: Option<ListTradesRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl TradesListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListTradesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: ListTradesRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn order(mut self, value: ListTradesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListTradesRequestDirection) -> Self {
        self.direction = Some(value);
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

    /// Consumes the builder and constructs a [`TradesListQueryRequest`].
    pub fn build(self) -> Result<TradesListQueryRequest, BuildError> {
        Ok(TradesListQueryRequest {
            account_id: self.account_id,
            status: self.status,
            r#type: self.r#type,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
