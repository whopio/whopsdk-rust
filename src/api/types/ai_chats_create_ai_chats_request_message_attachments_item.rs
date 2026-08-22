pub use crate::prelude::*;

/// Input for an attachment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAiChatsRequestMessageAttachmentsItem {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl CreateAiChatsRequestMessageAttachmentsItem {
    pub fn builder() -> CreateAiChatsRequestMessageAttachmentsItemBuilder {
        <CreateAiChatsRequestMessageAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAiChatsRequestMessageAttachmentsItemBuilder {
    id: Option<String>,
}

impl CreateAiChatsRequestMessageAttachmentsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAiChatsRequestMessageAttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateAiChatsRequestMessageAttachmentsItemBuilder::id)
    pub fn build(self) -> Result<CreateAiChatsRequestMessageAttachmentsItem, BuildError> {
        Ok(CreateAiChatsRequestMessageAttachmentsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
