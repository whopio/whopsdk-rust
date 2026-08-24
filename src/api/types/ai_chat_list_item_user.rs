pub use crate::prelude::*;

/// The user who owns this AI chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AiChatListItemUser {
    /// The unique identifier for the user.
    #[serde(default)]
    pub id: String,
}

impl AiChatListItemUser {
    pub fn builder() -> AiChatListItemUserBuilder {
        <AiChatListItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AiChatListItemUserBuilder {
    id: Option<String>,
}

impl AiChatListItemUserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AiChatListItemUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AiChatListItemUserBuilder::id)
    pub fn build(self) -> Result<AiChatListItemUser, BuildError> {
        Ok(AiChatListItemUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
