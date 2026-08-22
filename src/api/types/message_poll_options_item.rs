pub use crate::prelude::*;

/// Represents a single poll option
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessagePollOptionsItem {
    /// The unique identifier for the poll option.
    #[serde(default)]
    pub id: String,
    /// The text of the poll option
    #[serde(default)]
    pub text: String,
}

impl MessagePollOptionsItem {
    pub fn builder() -> MessagePollOptionsItemBuilder {
        <MessagePollOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessagePollOptionsItemBuilder {
    id: Option<String>,
    text: Option<String>,
}

impl MessagePollOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MessagePollOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MessagePollOptionsItemBuilder::id)
    /// - [`text`](MessagePollOptionsItemBuilder::text)
    pub fn build(self) -> Result<MessagePollOptionsItem, BuildError> {
        Ok(MessagePollOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
