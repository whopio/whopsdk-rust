pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteMembershipsRequest {
    /// Recipient email address. Pass exactly one of `user_id` or `email`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Recipient user ID, prefixed `user_`. Pass exactly one of `user_id` or `email`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Free variant whose membership the recipient is invited to, prefixed `plan_`.
    #[serde(default)]
    pub variant_id: String,
}

impl InviteMembershipsRequest {
    pub fn builder() -> InviteMembershipsRequestBuilder {
        <InviteMembershipsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteMembershipsRequestBuilder {
    email: Option<String>,
    user_id: Option<String>,
    variant_id: Option<String>,
}

impl InviteMembershipsRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InviteMembershipsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`variant_id`](InviteMembershipsRequestBuilder::variant_id)
    pub fn build(self) -> Result<InviteMembershipsRequest, BuildError> {
        Ok(InviteMembershipsRequest {
            email: self.email,
            user_id: self.user_id,
            variant_id: self
                .variant_id
                .ok_or_else(|| BuildError::missing_field("variant_id"))?,
        })
    }
}
