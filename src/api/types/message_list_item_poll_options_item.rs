pub use crate::prelude::*;

/// Represents a single poll option
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessageListItemPollOptionsItem {
    /// The unique identifier for the poll option.
    #[serde(default)]
    pub id: String,
    /// The text of the poll option
    #[serde(default)]
    pub text: String,
}

impl MessageListItemPollOptionsItem {
    pub fn builder() -> MessageListItemPollOptionsItemBuilder {
        <MessageListItemPollOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageListItemPollOptionsItemBuilder {
    id: Option<String>,
    text: Option<String>,
}

impl MessageListItemPollOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MessageListItemPollOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MessageListItemPollOptionsItemBuilder::id)
    /// - [`text`](MessageListItemPollOptionsItemBuilder::text)
    pub fn build(self) -> Result<MessageListItemPollOptionsItem, BuildError> {
        Ok(MessageListItemPollOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
