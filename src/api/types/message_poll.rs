pub use crate::prelude::*;

/// A poll attached to this message. Null if the message does not contain a poll.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessagePoll {
    /// The options for the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<MessagePollOptionsItem>>,
}

impl MessagePoll {
    pub fn builder() -> MessagePollBuilder {
        <MessagePollBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessagePollBuilder {
    options: Option<Vec<MessagePollOptionsItem>>,
}

impl MessagePollBuilder {
    pub fn options(mut self, value: Vec<MessagePollOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MessagePoll`].
    pub fn build(self) -> Result<MessagePoll, BuildError> {
        Ok(MessagePoll {
            options: self.options,
        })
    }
}
