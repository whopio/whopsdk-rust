pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FinancingApplicationsListQueryRequest {
    /// Only applications for this account, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only applications in this review state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListFinancingApplicationsRequestStatus>,
    /// Applications created at or before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Applications created at or after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Field to sort by. Defaults to created_at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListFinancingApplicationsRequestOrder>,
    /// Sort direction. Defaults to desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListFinancingApplicationsRequestDirection>,
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

impl FinancingApplicationsListQueryRequest {
    pub fn builder() -> FinancingApplicationsListQueryRequestBuilder {
        <FinancingApplicationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancingApplicationsListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListFinancingApplicationsRequestStatus>,
    created_before: Option<String>,
    created_after: Option<String>,
    order: Option<ListFinancingApplicationsRequestOrder>,
    direction: Option<ListFinancingApplicationsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl FinancingApplicationsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListFinancingApplicationsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListFinancingApplicationsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListFinancingApplicationsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`FinancingApplicationsListQueryRequest`].
    pub fn build(self) -> Result<FinancingApplicationsListQueryRequest, BuildError> {
        Ok(FinancingApplicationsListQueryRequest {
            account_id: self.account_id,
            status: self.status,
            created_before: self.created_before,
            created_after: self.created_after,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
