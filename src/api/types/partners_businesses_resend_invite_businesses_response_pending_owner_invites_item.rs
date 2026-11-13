pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendInviteBusinessesResponsePendingOwnerInvitesItem {
    /// The address the invite to claim this business was emailed to.
    #[serde(default)]
    pub email: String,
    /// The invite ID (an ausri_ identifier). Pass it to the resend endpoint.
    #[serde(default)]
    pub id: String,
}

impl ResendInviteBusinessesResponsePendingOwnerInvitesItem {
    pub fn builder() -> ResendInviteBusinessesResponsePendingOwnerInvitesItemBuilder {
        <ResendInviteBusinessesResponsePendingOwnerInvitesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesResponsePendingOwnerInvitesItemBuilder {
    email: Option<String>,
    id: Option<String>,
}

impl ResendInviteBusinessesResponsePendingOwnerInvitesItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesResponsePendingOwnerInvitesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](ResendInviteBusinessesResponsePendingOwnerInvitesItemBuilder::email)
    /// - [`id`](ResendInviteBusinessesResponsePendingOwnerInvitesItemBuilder::id)
    pub fn build(
        self,
    ) -> Result<ResendInviteBusinessesResponsePendingOwnerInvitesItem, BuildError> {
        Ok(ResendInviteBusinessesResponsePendingOwnerInvitesItem {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
