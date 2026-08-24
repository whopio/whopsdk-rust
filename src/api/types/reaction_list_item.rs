pub use crate::prelude::*;

/// A single reaction left by a user on a feed post, such as a like or emoji.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReactionListItem {
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
    pub user: ReactionListItemUser,
}

impl ReactionListItem {
    pub fn builder() -> ReactionListItemBuilder {
        <ReactionListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReactionListItemBuilder {
    emoji: Option<String>,
    id: Option<String>,
    resource_id: Option<String>,
    user: Option<ReactionListItemUser>,
}

impl ReactionListItemBuilder {
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

    pub fn user(mut self, value: ReactionListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReactionListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReactionListItemBuilder::id)
    /// - [`resource_id`](ReactionListItemBuilder::resource_id)
    /// - [`user`](ReactionListItemBuilder::user)
    pub fn build(self) -> Result<ReactionListItem, BuildError> {
        Ok(ReactionListItem {
            emoji: self.emoji,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
