pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveBusinessesResponsePendingOwnerInvitesItem {
    /// The address the invite to claim this business was emailed to.
    #[serde(default)]
    pub email: String,
    /// The invite ID (an ausri_ identifier). Pass it to the resend endpoint.
    #[serde(default)]
    pub id: String,
}

impl RetrieveBusinessesResponsePendingOwnerInvitesItem {
    pub fn builder() -> RetrieveBusinessesResponsePendingOwnerInvitesItemBuilder {
        <RetrieveBusinessesResponsePendingOwnerInvitesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponsePendingOwnerInvitesItemBuilder {
    email: Option<String>,
    id: Option<String>,
}

impl RetrieveBusinessesResponsePendingOwnerInvitesItemBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponsePendingOwnerInvitesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](RetrieveBusinessesResponsePendingOwnerInvitesItemBuilder::email)
    /// - [`id`](RetrieveBusinessesResponsePendingOwnerInvitesItemBuilder::id)
    pub fn build(self) -> Result<RetrieveBusinessesResponsePendingOwnerInvitesItem, BuildError> {
        Ok(RetrieveBusinessesResponsePendingOwnerInvitesItem {
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
