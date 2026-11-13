pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChatMessagePollOption {
    /// Option identifier within the poll.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub vote_count: i64,
    /// Whether you voted for this option.
    #[serde(default)]
    pub voted: bool,
}

impl ChatMessagePollOption {
    pub fn builder() -> ChatMessagePollOptionBuilder {
        <ChatMessagePollOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatMessagePollOptionBuilder {
    id: Option<String>,
    text: Option<String>,
    vote_count: Option<i64>,
    voted: Option<bool>,
}

impl ChatMessagePollOptionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn vote_count(mut self, value: i64) -> Self {
        self.vote_count = Some(value);
        self
    }

    pub fn voted(mut self, value: bool) -> Self {
        self.voted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatMessagePollOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChatMessagePollOptionBuilder::id)
    /// - [`text`](ChatMessagePollOptionBuilder::text)
    /// - [`vote_count`](ChatMessagePollOptionBuilder::vote_count)
    /// - [`voted`](ChatMessagePollOptionBuilder::voted)
    pub fn build(self) -> Result<ChatMessagePollOption, BuildError> {
        Ok(ChatMessagePollOption {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            vote_count: self
                .vote_count
                .ok_or_else(|| BuildError::missing_field("vote_count"))?,
            voted: self
                .voted
                .ok_or_else(|| BuildError::missing_field("voted"))?,
        })
    }
}
