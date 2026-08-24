pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteMembershipsRequestBodyUserId {
    /// Free plan whose membership the recipient is invited to, prefixed `plan_`.
    #[serde(default)]
    pub plan_id: String,
    /// Recipient user ID, prefixed `user_`.
    #[serde(default)]
    pub user_id: String,
}

impl InviteMembershipsRequestBodyUserId {
    pub fn builder() -> InviteMembershipsRequestBodyUserIdBuilder {
        <InviteMembershipsRequestBodyUserIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteMembershipsRequestBodyUserIdBuilder {
    plan_id: Option<String>,
    user_id: Option<String>,
}

impl InviteMembershipsRequestBodyUserIdBuilder {
    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InviteMembershipsRequestBodyUserId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan_id`](InviteMembershipsRequestBodyUserIdBuilder::plan_id)
    /// - [`user_id`](InviteMembershipsRequestBodyUserIdBuilder::user_id)
    pub fn build(self) -> Result<InviteMembershipsRequestBodyUserId, BuildError> {
        Ok(InviteMembershipsRequestBodyUserId {
            plan_id: self
                .plan_id
                .ok_or_else(|| BuildError::missing_field("plan_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
