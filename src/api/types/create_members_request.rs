pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMembersRequest {
    /// The account to add the member to (`biz_` tag). Defaults to the account the credential acts as.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The email address of the person to add. If no Whop user has it, one is created.
    #[serde(default)]
    pub email: String,
    /// The name of the person to add. Ignored when a user on Whop already exists for this email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateMembersRequest {
    pub fn builder() -> CreateMembersRequestBuilder {
        <CreateMembersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMembersRequestBuilder {
    account_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
}

impl CreateMembersRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMembersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](CreateMembersRequestBuilder::email)
    pub fn build(self) -> Result<CreateMembersRequest, BuildError> {
        Ok(CreateMembersRequest {
            account_id: self.account_id,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
        })
    }
}
