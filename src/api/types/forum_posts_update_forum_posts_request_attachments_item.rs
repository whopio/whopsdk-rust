pub use crate::prelude::*;

/// Input for an attachment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateForumPostsRequestAttachmentsItem {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateForumPostsRequestAttachmentsItem {
    pub fn builder() -> UpdateForumPostsRequestAttachmentsItemBuilder {
        <UpdateForumPostsRequestAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateForumPostsRequestAttachmentsItemBuilder {
    id: Option<String>,
}

impl UpdateForumPostsRequestAttachmentsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateForumPostsRequestAttachmentsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateForumPostsRequestAttachmentsItemBuilder::id)
    pub fn build(self) -> Result<UpdateForumPostsRequestAttachmentsItem, BuildError> {
        Ok(UpdateForumPostsRequestAttachmentsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
