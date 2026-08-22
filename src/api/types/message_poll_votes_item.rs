pub use crate::prelude::*;

/// Represents a reaction count for a feed post
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessagePollVotesItem {
    /// The number of users who reacted
    #[serde(default)]
    pub count: i64,
    /// The reaction that was used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_id: Option<String>,
}

impl MessagePollVotesItem {
    pub fn builder() -> MessagePollVotesItemBuilder {
        <MessagePollVotesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessagePollVotesItemBuilder {
    count: Option<i64>,
    option_id: Option<String>,
}

impl MessagePollVotesItemBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn option_id(mut self, value: impl Into<String>) -> Self {
        self.option_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MessagePollVotesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`count`](MessagePollVotesItemBuilder::count)
    pub fn build(self) -> Result<MessagePollVotesItem, BuildError> {
        Ok(MessagePollVotesItem {
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            option_id: self.option_id,
        })
    }
}
