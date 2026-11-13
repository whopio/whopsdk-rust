pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMessagesRequestPollOptionsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub text: String,
}

impl CreateMessagesRequestPollOptionsItem {
    pub fn builder() -> CreateMessagesRequestPollOptionsItemBuilder {
        <CreateMessagesRequestPollOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMessagesRequestPollOptionsItemBuilder {
    id: Option<String>,
    text: Option<String>,
}

impl CreateMessagesRequestPollOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMessagesRequestPollOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateMessagesRequestPollOptionsItemBuilder::id)
    /// - [`text`](CreateMessagesRequestPollOptionsItemBuilder::text)
    pub fn build(self) -> Result<CreateMessagesRequestPollOptionsItem, BuildError> {
        Ok(CreateMessagesRequestPollOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
