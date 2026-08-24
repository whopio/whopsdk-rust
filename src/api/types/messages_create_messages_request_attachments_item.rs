pub use crate::prelude::*;

/// Input for an attachment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMessagesRequestAttachmentsItem {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl CreateMessagesRequestAttachmentsItem {
    pub fn builder() -> CreateMessagesRequestAttachmentsItemBuilder {
        <CreateMessagesRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMessagesRequestAttachmentsItemBuilder {
    id: Option<String>,
}

impl CreateMessagesRequestAttachmentsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMessagesRequestAttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateMessagesRequestAttachmentsItemBuilder::id)
    pub fn build(self) -> Result<CreateMessagesRequestAttachmentsItem, BuildError> {
        Ok(CreateMessagesRequestAttachmentsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
