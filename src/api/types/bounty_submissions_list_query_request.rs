pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BountySubmissionsListQueryRequest {
    /// Scope the list to submissions on this account's bounties (`biz_` tag). Requires read access to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only submissions on this bounty (`bnty_` tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounty_id: Option<String>,
    /// Filter by lifecycle state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListBountySubmissionsRequestStatus>,
    /// Only submissions created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only submissions created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListBountySubmissionsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListBountySubmissionsRequestDirection>,
    /// Number of submissions to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to paginate forwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of submissions to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to paginate backwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl BountySubmissionsListQueryRequest {
    pub fn builder() -> BountySubmissionsListQueryRequestBuilder {
        <BountySubmissionsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountySubmissionsListQueryRequestBuilder {
    account_id: Option<String>,
    bounty_id: Option<String>,
    status: Option<ListBountySubmissionsRequestStatus>,
    created_after: Option<String>,
    created_before: Option<String>,
    order: Option<ListBountySubmissionsRequestOrder>,
    direction: Option<ListBountySubmissionsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl BountySubmissionsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn bounty_id(mut self, value: impl Into<String>) -> Self {
        self.bounty_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListBountySubmissionsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListBountySubmissionsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListBountySubmissionsRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`BountySubmissionsListQueryRequest`].
    pub fn build(self) -> Result<BountySubmissionsListQueryRequest, BuildError> {
        Ok(BountySubmissionsListQueryRequest {
            account_id: self.account_id,
            bounty_id: self.bounty_id,
            status: self.status,
            created_after: self.created_after,
            created_before: self.created_before,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
