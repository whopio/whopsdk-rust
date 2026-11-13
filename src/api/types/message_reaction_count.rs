pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MessageReactionCount {
    pub kind: MessageReactionCountKind,
    /// Whether you reacted with this value.
    #[serde(default)]
    pub reacted: bool,
    #[serde(default)]
    pub total_score: i64,
    #[serde(default)]
    pub user_count: i64,
    /// The Unicode emoji for an emoji reaction. For other reaction kinds, the reaction's value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl MessageReactionCount {
    pub fn builder() -> MessageReactionCountBuilder {
        <MessageReactionCountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageReactionCountBuilder {
    kind: Option<MessageReactionCountKind>,
    reacted: Option<bool>,
    total_score: Option<i64>,
    user_count: Option<i64>,
    value: Option<String>,
}

impl MessageReactionCountBuilder {
    pub fn kind(mut self, value: MessageReactionCountKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn reacted(mut self, value: bool) -> Self {
        self.reacted = Some(value);
        self
    }

    pub fn total_score(mut self, value: i64) -> Self {
        self.total_score = Some(value);
        self
    }

    pub fn user_count(mut self, value: i64) -> Self {
        self.user_count = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MessageReactionCount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](MessageReactionCountBuilder::kind)
    /// - [`reacted`](MessageReactionCountBuilder::reacted)
    /// - [`total_score`](MessageReactionCountBuilder::total_score)
    /// - [`user_count`](MessageReactionCountBuilder::user_count)
    pub fn build(self) -> Result<MessageReactionCount, BuildError> {
        Ok(MessageReactionCount {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            reacted: self
                .reacted
                .ok_or_else(|| BuildError::missing_field("reacted"))?,
            total_score: self
                .total_score
                .ok_or_else(|| BuildError::missing_field("total_score"))?,
            user_count: self
                .user_count
                .ok_or_else(|| BuildError::missing_field("user_count"))?,
            value: self.value,
        })
    }
}
