pub use crate::prelude::*;

/// A poll to attach to this post, allowing members to vote on options.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateForumPostsRequestPoll {
    /// The options for the poll. Must have sequential IDs starting from 1
    #[serde(default)]
    pub options: Vec<CreateForumPostsRequestPollOptionsItem>,
}

impl CreateForumPostsRequestPoll {
    pub fn builder() -> CreateForumPostsRequestPollBuilder {
        <CreateForumPostsRequestPollBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateForumPostsRequestPollBuilder {
    options: Option<Vec<CreateForumPostsRequestPollOptionsItem>>,
}

impl CreateForumPostsRequestPollBuilder {
    pub fn options(mut self, value: Vec<CreateForumPostsRequestPollOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateForumPostsRequestPoll`].
    /// This method will fail if any of the following fields are not set:
    /// - [`options`](CreateForumPostsRequestPollBuilder::options)
    pub fn build(self) -> Result<CreateForumPostsRequestPoll, BuildError> {
        Ok(CreateForumPostsRequestPoll {
            options: self
                .options
                .ok_or_else(|| BuildError::missing_field("options"))?,
        })
    }
}
