pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTeamMembersRequest {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Email address to invite. Mutually exclusive with `user_id`. If the email already belongs to a Whop account it is treated the same as passing that account's `user_id`; otherwise a pending invite is created for the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The system role to grant.
    pub role: CreateTeamMembersRequestRole,
    /// The user to add to the team, prefixed `user_`. Mutually exclusive with `email`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreateTeamMembersRequest {
    pub fn builder() -> CreateTeamMembersRequestBuilder {
        <CreateTeamMembersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTeamMembersRequestBuilder {
    account_id: Option<String>,
    email: Option<String>,
    role: Option<CreateTeamMembersRequestRole>,
    user_id: Option<String>,
}

impl CreateTeamMembersRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: CreateTeamMembersRequestRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTeamMembersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateTeamMembersRequestBuilder::account_id)
    /// - [`role`](CreateTeamMembersRequestBuilder::role)
    pub fn build(self) -> Result<CreateTeamMembersRequest, BuildError> {
        Ok(CreateTeamMembersRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            email: self.email,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            user_id: self.user_id,
        })
    }
}
