pub use crate::prelude::*;

/// A poll attached to this message. Null if the message does not contain a poll.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SupportChannelListItemLastMessagePoll {
    /// The options for the poll
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<SupportChannelListItemLastMessagePollOptionsItem>>,
}

impl SupportChannelListItemLastMessagePoll {
    pub fn builder() -> SupportChannelListItemLastMessagePollBuilder {
        <SupportChannelListItemLastMessagePollBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SupportChannelListItemLastMessagePollBuilder {
    options: Option<Vec<SupportChannelListItemLastMessagePollOptionsItem>>,
}

impl SupportChannelListItemLastMessagePollBuilder {
    pub fn options(mut self, value: Vec<SupportChannelListItemLastMessagePollOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SupportChannelListItemLastMessagePoll`].
    pub fn build(self) -> Result<SupportChannelListItemLastMessagePoll, BuildError> {
        Ok(SupportChannelListItemLastMessagePoll {
            options: self.options,
        })
    }
}
