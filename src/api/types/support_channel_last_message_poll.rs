pub use crate::prelude::*;

/// A poll attached to this message. Null if the message does not contain a poll.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelLastMessagePoll {
    /// The options for the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SupportChannelLastMessagePollOptionsItem>>,
}

impl SupportChannelLastMessagePoll {
    pub fn builder() -> SupportChannelLastMessagePollBuilder {
        <SupportChannelLastMessagePollBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelLastMessagePollBuilder {
    options: Option<Vec<SupportChannelLastMessagePollOptionsItem>>,
}

impl SupportChannelLastMessagePollBuilder {
    pub fn options(mut self, value: Vec<SupportChannelLastMessagePollOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelLastMessagePoll`].
    pub fn build(self) -> Result<SupportChannelLastMessagePoll, BuildError> {
        Ok(SupportChannelLastMessagePoll {
            options: self.options,
        })
    }
}
