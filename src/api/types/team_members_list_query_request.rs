pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamMembersListQueryRequest {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Only return members with this status: `joined` (accepted members) or `pending` (pending invites). Both are returned by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListTeamMembersRequestStatus>,
    /// Only return the membership for this user ID, prefixed `user_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Only return members with this role. `custom` matches members on a dashboard-managed custom role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ListTeamMembersRequestRole>,
    /// Only return members added before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return members added after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Field used to sort members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListTeamMembersRequestOrder>,
    /// Sort direction. Defaults to `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListTeamMembersRequestDirection>,
    /// Number of members to return. Defaults to 20; maximum 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor for the next page of members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of members to return from the end of the window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Cursor to paginate backwards from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl TeamMembersListQueryRequest {
    pub fn builder() -> TeamMembersListQueryRequestBuilder {
        <TeamMembersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamMembersListQueryRequestBuilder {
    account_id: Option<String>,
    status: Option<ListTeamMembersRequestStatus>,
    user_id: Option<String>,
    role: Option<ListTeamMembersRequestRole>,
    created_before: Option<String>,
    created_after: Option<String>,
    order: Option<ListTeamMembersRequestOrder>,
    direction: Option<ListTeamMembersRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl TeamMembersListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListTeamMembersRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: ListTeamMembersRequestRole) -> Self {
        self.role = Some(value);
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

    pub fn order(mut self, value: ListTeamMembersRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListTeamMembersRequestDirection) -> Self {
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

    /// Consumes the builder and constructs a [`TeamMembersListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](TeamMembersListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<TeamMembersListQueryRequest, BuildError> {
        Ok(TeamMembersListQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            status: self.status,
            user_id: self.user_id,
            role: self.role,
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
