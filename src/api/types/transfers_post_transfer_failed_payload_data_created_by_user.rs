pub use crate::prelude::*;

/// The user who initiated the transfer, such as the team member who sent a manual payout. Null if the creator is unavailable.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostTransferFailedPayloadDataCreatedByUser {
    /// User ID.
    #[serde(default)]
    pub id: String,
    /// User display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User's username.
    #[serde(default)]
    pub username: String,
}

impl PostTransferFailedPayloadDataCreatedByUser {
    pub fn builder() -> PostTransferFailedPayloadDataCreatedByUserBuilder {
        <PostTransferFailedPayloadDataCreatedByUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostTransferFailedPayloadDataCreatedByUserBuilder {
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl PostTransferFailedPayloadDataCreatedByUserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostTransferFailedPayloadDataCreatedByUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PostTransferFailedPayloadDataCreatedByUserBuilder::id)
    /// - [`username`](PostTransferFailedPayloadDataCreatedByUserBuilder::username)
    pub fn build(self) -> Result<PostTransferFailedPayloadDataCreatedByUser, BuildError> {
        Ok(PostTransferFailedPayloadDataCreatedByUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
