pub use crate::prelude::*;

/// Query parameters for referredUsers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReferredUsersQueryRequest {
    /// When true, only referred users who brought at least one business onto Whop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_businesses: Option<bool>,
    /// When true, only referred users with at least one business that has generated earnings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_earning_businesses: Option<bool>,
    /// Number of referred users to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of referred users to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to fetch the page before (from page_info.start_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl ReferredUsersQueryRequest {
    pub fn builder() -> ReferredUsersQueryRequestBuilder {
        <ReferredUsersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReferredUsersQueryRequestBuilder {
    has_businesses: Option<bool>,
    has_earning_businesses: Option<bool>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl ReferredUsersQueryRequestBuilder {
    pub fn has_businesses(mut self, value: bool) -> Self {
        self.has_businesses = Some(value);
        self
    }

    pub fn has_earning_businesses(mut self, value: bool) -> Self {
        self.has_earning_businesses = Some(value);
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

    /// Consumes the builder and constructs a [`ReferredUsersQueryRequest`].
    pub fn build(self) -> Result<ReferredUsersQueryRequest, BuildError> {
        Ok(ReferredUsersQueryRequest {
            has_businesses: self.has_businesses,
            has_earning_businesses: self.has_earning_businesses,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
