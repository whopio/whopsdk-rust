pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainsListQueryRequest {
    /// Only domains belonging to this account, prefixed biz_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only domains assigned to this app, prefixed app_.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Only domains with this lifecycle status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListDomainsRequestStatus>,
    /// Field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListDomainsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListDomainsRequestDirection>,
    /// Number of domains from the start of the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor for the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of domains from the end of the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor for the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl DomainsListQueryRequest {
    pub fn builder() -> DomainsListQueryRequestBuilder {
        <DomainsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainsListQueryRequestBuilder {
    account_id: Option<String>,
    app_id: Option<String>,
    status: Option<ListDomainsRequestStatus>,
    order: Option<ListDomainsRequestOrder>,
    direction: Option<ListDomainsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl DomainsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListDomainsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn order(mut self, value: ListDomainsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListDomainsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`DomainsListQueryRequest`].
    pub fn build(self) -> Result<DomainsListQueryRequest, BuildError> {
        Ok(DomainsListQueryRequest {
            account_id: self.account_id,
            app_id: self.app_id,
            status: self.status,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
