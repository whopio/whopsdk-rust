pub use crate::prelude::*;

/// A single reaction left by a user on a feed post, such as a like or emoji.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Reaction {
    /// The emoji used for this reaction in shortcode format. Null if the reaction type is not emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    /// The unique identifier for the entity
    #[serde(default)]
    pub id: String,
    /// The unique identifier of the post this reaction was left on.
    #[serde(default)]
    pub resource_id: String,
    /// The user who left this reaction on the post.
    #[serde(default)]
    pub user: ReactionUser,
}

impl Reaction {
    pub fn builder() -> ReactionBuilder {
        <ReactionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReactionBuilder {
    emoji: Option<String>,
    id: Option<String>,
    resource_id: Option<String>,
    user: Option<ReactionUser>,
}

impl ReactionBuilder {
    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn user(mut self, value: ReactionUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Reaction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReactionBuilder::id)
    /// - [`resource_id`](ReactionBuilder::resource_id)
    /// - [`user`](ReactionBuilder::user)
    pub fn build(self) -> Result<Reaction, BuildError> {
        Ok(Reaction {
            emoji: self.emoji,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
