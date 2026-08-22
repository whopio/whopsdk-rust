pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembersListQueryRequest {
    /// The account to list members for (`biz_` tag). Defaults to the account the credential acts as.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Filter by what the member can reach on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<ListMembersRequestAccessLevel>,
    /// Filter by whether the member is still part of the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListMembersRequestStatus>,
    /// Search members by name or username. An exact email address also matches when the credential holds the member:email:read scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only members who joined after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only members who joined before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListMembersRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListMembersRequestDirection>,
    /// Number of members to return from the start of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to paginate forwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of members to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to paginate backwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl MembersListQueryRequest {
    pub fn builder() -> MembersListQueryRequestBuilder {
        <MembersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembersListQueryRequestBuilder {
    account_id: Option<String>,
    access_level: Option<ListMembersRequestAccessLevel>,
    status: Option<ListMembersRequestStatus>,
    query: Option<String>,
    created_after: Option<String>,
    created_before: Option<String>,
    order: Option<ListMembersRequestOrder>,
    direction: Option<ListMembersRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl MembersListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn access_level(mut self, value: ListMembersRequestAccessLevel) -> Self {
        self.access_level = Some(value);
        self
    }

    pub fn status(mut self, value: ListMembersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
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

    pub fn order(mut self, value: ListMembersRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListMembersRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`MembersListQueryRequest`].
    pub fn build(self) -> Result<MembersListQueryRequest, BuildError> {
        Ok(MembersListQueryRequest {
            account_id: self.account_id,
            access_level: self.access_level,
            status: self.status,
            query: self.query,
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
