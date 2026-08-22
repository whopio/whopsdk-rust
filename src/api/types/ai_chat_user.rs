pub use crate::prelude::*;

/// The user who owns this AI chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AiChatUser {
    /// The unique identifier for the user.
    #[serde(default)]
    pub id: String,
}

impl AiChatUser {
    pub fn builder() -> AiChatUserBuilder {
        <AiChatUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AiChatUserBuilder {
    id: Option<String>,
}

impl AiChatUserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AiChatUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AiChatUserBuilder::id)
    pub fn build(self) -> Result<AiChatUser, BuildError> {
        Ok(AiChatUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
