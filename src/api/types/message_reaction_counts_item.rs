pub use crate::prelude::*;

/// Represents a reaction count for a feed post
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessageReactionCountsItem {
    /// The number of users who reacted
    #[serde(default)]
    pub count: i64,
    /// The emoji that was used in shortcode format (:heart:)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
}

impl MessageReactionCountsItem {
    pub fn builder() -> MessageReactionCountsItemBuilder {
        <MessageReactionCountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageReactionCountsItemBuilder {
    count: Option<i64>,
    emoji: Option<String>,
}

impl MessageReactionCountsItemBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MessageReactionCountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](MessageReactionCountsItemBuilder::count)
    pub fn build(self) -> Result<MessageReactionCountsItem, BuildError> {
        Ok(MessageReactionCountsItem {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            emoji: self.emoji,
        })
    }
}
