pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChatReaction {
    /// When the user reacted, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The Unicode emoji, including any skin tone or variation selectors.
    #[serde(default)]
    pub emoji: String,
    /// Public profile of the user who reacted.
    #[serde(default)]
    pub user: UserSummary,
    /// ID of the user who reacted, prefixed `user_`.
    #[serde(default)]
    pub user_id: String,
}

impl ChatReaction {
    pub fn builder() -> ChatReactionBuilder {
        <ChatReactionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatReactionBuilder {
    created_at: Option<String>,
    emoji: Option<String>,
    user: Option<UserSummary>,
    user_id: Option<String>,
}

impl ChatReactionBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    pub fn user(mut self, value: UserSummary) -> Self {
        self.user = Some(value);
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatReaction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ChatReactionBuilder::created_at)
    /// - [`emoji`](ChatReactionBuilder::emoji)
    /// - [`user`](ChatReactionBuilder::user)
    /// - [`user_id`](ChatReactionBuilder::user_id)
    pub fn build(self) -> Result<ChatReaction, BuildError> {
        Ok(ChatReaction {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            emoji: self
                .emoji
                .ok_or_else(|| BuildError::missing_field("emoji"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
