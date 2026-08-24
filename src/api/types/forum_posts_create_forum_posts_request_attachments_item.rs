pub use crate::prelude::*;

/// Input for an attachment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateForumPostsRequestAttachmentsItem {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl CreateForumPostsRequestAttachmentsItem {
    pub fn builder() -> CreateForumPostsRequestAttachmentsItemBuilder {
        <CreateForumPostsRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateForumPostsRequestAttachmentsItemBuilder {
    id: Option<String>,
}

impl CreateForumPostsRequestAttachmentsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateForumPostsRequestAttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateForumPostsRequestAttachmentsItemBuilder::id)
    pub fn build(self) -> Result<CreateForumPostsRequestAttachmentsItem, BuildError> {
        Ok(CreateForumPostsRequestAttachmentsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
