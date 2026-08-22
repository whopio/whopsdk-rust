pub use crate::prelude::*;

/// Input for an attachment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMessagesRequestAttachmentsItem {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateMessagesRequestAttachmentsItem {
    pub fn builder() -> UpdateMessagesRequestAttachmentsItemBuilder {
        <UpdateMessagesRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMessagesRequestAttachmentsItemBuilder {
    id: Option<String>,
}

impl UpdateMessagesRequestAttachmentsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMessagesRequestAttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateMessagesRequestAttachmentsItemBuilder::id)
    pub fn build(self) -> Result<UpdateMessagesRequestAttachmentsItem, BuildError> {
        Ok(UpdateMessagesRequestAttachmentsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
