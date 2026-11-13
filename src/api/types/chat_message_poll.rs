pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChatMessagePoll {
    #[serde(default)]
    pub options: Vec<ChatMessagePollOption>,
}

impl ChatMessagePoll {
    pub fn builder() -> ChatMessagePollBuilder {
        <ChatMessagePollBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatMessagePollBuilder {
    options: Option<Vec<ChatMessagePollOption>>,
}

impl ChatMessagePollBuilder {
    pub fn options(mut self, value: Vec<ChatMessagePollOption>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatMessagePoll`].
    /// This method will fail if any of the following fields are not set:
    /// - [`options`](ChatMessagePollBuilder::options)
    pub fn build(self) -> Result<ChatMessagePoll, BuildError> {
        Ok(ChatMessagePoll {
            options: self
                .options
                .ok_or_else(|| BuildError::missing_field("options"))?,
        })
    }
}
