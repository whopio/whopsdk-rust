pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InviteMembershipsRequestBodyEmail {
    /// Recipient email address.
    #[serde(default)]
    pub email: String,
    /// Free plan whose membership the recipient is invited to, prefixed `plan_`.
    #[serde(default)]
    pub plan_id: String,
}

impl InviteMembershipsRequestBodyEmail {
    pub fn builder() -> InviteMembershipsRequestBodyEmailBuilder {
        <InviteMembershipsRequestBodyEmailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InviteMembershipsRequestBodyEmailBuilder {
    email: Option<String>,
    plan_id: Option<String>,
}

impl InviteMembershipsRequestBodyEmailBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InviteMembershipsRequestBodyEmail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](InviteMembershipsRequestBodyEmailBuilder::email)
    /// - [`plan_id`](InviteMembershipsRequestBodyEmailBuilder::plan_id)
    pub fn build(self) -> Result<InviteMembershipsRequestBodyEmail, BuildError> {
        Ok(InviteMembershipsRequestBodyEmail {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            plan_id: self
                .plan_id
                .ok_or_else(|| BuildError::missing_field("plan_id"))?,
        })
    }
}
