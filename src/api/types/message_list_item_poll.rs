pub use crate::prelude::*;

/// A poll attached to this message. Null if the message does not contain a poll.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessageListItemPoll {
    /// The options for the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<MessageListItemPollOptionsItem>>,
}

impl MessageListItemPoll {
    pub fn builder() -> MessageListItemPollBuilder {
        <MessageListItemPollBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageListItemPollBuilder {
    options: Option<Vec<MessageListItemPollOptionsItem>>,
}

impl MessageListItemPollBuilder {
    pub fn options(mut self, value: Vec<MessageListItemPollOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessageListItemPoll`].
    pub fn build(self) -> Result<MessageListItemPoll, BuildError> {
        Ok(MessageListItemPoll {
            options: self.options,
        })
    }
}
