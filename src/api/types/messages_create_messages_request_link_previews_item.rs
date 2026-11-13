pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMessagesRequestLinkPreviewsItem {
    /// URL to preview.
    #[serde(default)]
    pub url: String,
}

impl CreateMessagesRequestLinkPreviewsItem {
    pub fn builder() -> CreateMessagesRequestLinkPreviewsItemBuilder {
        <CreateMessagesRequestLinkPreviewsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMessagesRequestLinkPreviewsItemBuilder {
    url: Option<String>,
}

impl CreateMessagesRequestLinkPreviewsItemBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMessagesRequestLinkPreviewsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateMessagesRequestLinkPreviewsItemBuilder::url)
    pub fn build(self) -> Result<CreateMessagesRequestLinkPreviewsItem, BuildError> {
        Ok(CreateMessagesRequestLinkPreviewsItem {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
