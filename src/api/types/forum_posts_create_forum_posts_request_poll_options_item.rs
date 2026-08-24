pub use crate::prelude::*;

/// Input type for a single poll option
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateForumPostsRequestPollOptionsItem {
    /// Sequential ID for the poll option (starting from '1')
    #[serde(default)]
    pub id: String,
    /// The text of the poll option
    #[serde(default)]
    pub text: String,
}

impl CreateForumPostsRequestPollOptionsItem {
    pub fn builder() -> CreateForumPostsRequestPollOptionsItemBuilder {
        <CreateForumPostsRequestPollOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateForumPostsRequestPollOptionsItemBuilder {
    id: Option<String>,
    text: Option<String>,
}

impl CreateForumPostsRequestPollOptionsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateForumPostsRequestPollOptionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateForumPostsRequestPollOptionsItemBuilder::id)
    /// - [`text`](CreateForumPostsRequestPollOptionsItemBuilder::text)
    pub fn build(self) -> Result<CreateForumPostsRequestPollOptionsItem, BuildError> {
        Ok(CreateForumPostsRequestPollOptionsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
