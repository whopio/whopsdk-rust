pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthorizedUsersListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Filter results to a specific user to check if they are an authorized team member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<AuthorizedUserRoles>,
    /// Only return authorized users created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only return authorized users created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// The unique identifier of the company to list authorized users for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl AuthorizedUsersListQueryRequest {
    pub fn builder() -> AuthorizedUsersListQueryRequestBuilder {
        <AuthorizedUsersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthorizedUsersListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    user_id: Option<String>,
    role: Option<AuthorizedUserRoles>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    account_id: Option<String>,
}

impl AuthorizedUsersListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn role(mut self, value: AuthorizedUserRoles) -> Self {
        self.role = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthorizedUsersListQueryRequest`].
    pub fn build(self) -> Result<AuthorizedUsersListQueryRequest, BuildError> {
        Ok(AuthorizedUsersListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            user_id: self.user_id,
            role: self.role,
            created_before: self.created_before,
            created_after: self.created_after,
            account_id: self.account_id,
        })
    }
}
