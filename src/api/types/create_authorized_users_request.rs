pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAuthorizedUsersRequest {
    /// The ID of the company to add the authorized user to.
    #[serde(default)]
    pub account_id: String,
    /// Re-authentication proof required to perform this sensitive action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<CreateAuthorizedUsersRequestElevation>,
    /// The role to assign to the authorized user within the company. Supported roles: 'moderator', 'sales_manager'.
    pub role: GrantableAuthorizedUserRoles,
    /// Whether to send notification emails to the user on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_emails: Option<bool>,
    /// The ID of the user to add as an authorized user.
    #[serde(default)]
    pub user_id: String,
}

impl CreateAuthorizedUsersRequest {
    pub fn builder() -> CreateAuthorizedUsersRequestBuilder {
        <CreateAuthorizedUsersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAuthorizedUsersRequestBuilder {
    account_id: Option<String>,
    elevation: Option<CreateAuthorizedUsersRequestElevation>,
    role: Option<GrantableAuthorizedUserRoles>,
    send_emails: Option<bool>,
    user_id: Option<String>,
}

impl CreateAuthorizedUsersRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn elevation(mut self, value: CreateAuthorizedUsersRequestElevation) -> Self {
        self.elevation = Some(value);
        self
    }

    pub fn role(mut self, value: GrantableAuthorizedUserRoles) -> Self {
        self.role = Some(value);
        self
    }

    pub fn send_emails(mut self, value: bool) -> Self {
        self.send_emails = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAuthorizedUsersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateAuthorizedUsersRequestBuilder::account_id)
    /// - [`role`](CreateAuthorizedUsersRequestBuilder::role)
    /// - [`user_id`](CreateAuthorizedUsersRequestBuilder::user_id)
    pub fn build(self) -> Result<CreateAuthorizedUsersRequest, BuildError> {
        Ok(CreateAuthorizedUsersRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            elevation: self.elevation,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            send_emails: self.send_emails,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
