pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMessagesRequestLinkPreviewsItem {
    /// URL to preview.
    #[serde(default)]
    pub url: String,
}

impl UpdateMessagesRequestLinkPreviewsItem {
    pub fn builder() -> UpdateMessagesRequestLinkPreviewsItemBuilder {
        <UpdateMessagesRequestLinkPreviewsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMessagesRequestLinkPreviewsItemBuilder {
    url: Option<String>,
}

impl UpdateMessagesRequestLinkPreviewsItemBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMessagesRequestLinkPreviewsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](UpdateMessagesRequestLinkPreviewsItemBuilder::url)
    pub fn build(self) -> Result<UpdateMessagesRequestLinkPreviewsItem, BuildError> {
        Ok(UpdateMessagesRequestLinkPreviewsItem {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
