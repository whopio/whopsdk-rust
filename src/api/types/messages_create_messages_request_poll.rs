pub use crate::prelude::*;

/// A poll to attach to this message, allowing recipients to vote on options.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMessagesRequestPoll {
    /// The options for the poll. Must have sequential IDs starting from 1
    #[serde(default)]
    pub options: Vec<CreateMessagesRequestPollOptionsItem>,
}

impl CreateMessagesRequestPoll {
    pub fn builder() -> CreateMessagesRequestPollBuilder {
        <CreateMessagesRequestPollBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMessagesRequestPollBuilder {
    options: Option<Vec<CreateMessagesRequestPollOptionsItem>>,
}

impl CreateMessagesRequestPollBuilder {
    pub fn options(mut self, value: Vec<CreateMessagesRequestPollOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateMessagesRequestPoll`].
    /// This method will fail if any of the following fields are not set:
    /// - [`options`](CreateMessagesRequestPollBuilder::options)
    pub fn build(self) -> Result<CreateMessagesRequestPoll, BuildError> {
        Ok(CreateMessagesRequestPoll {
            options: self
                .options
                .ok_or_else(|| BuildError::missing_field("options"))?,
        })
    }
}
