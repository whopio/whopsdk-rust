pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMessagesRequest {
    /// Replacement attachment list; pass an empty list to remove attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<UpdateMessagesRequestAttachmentsItem>>,
    /// Replacement Markdown content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Up to 10 GIFs to attach. URLs must use HTTPS on static.klipy.com, static1.klipy.com, or static2.klipy.com. Omit to preserve on edit; an empty list clears them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gifs: Option<Vec<UpdateMessagesRequestGifsItem>>,
    /// Replacement preview URLs; pass an empty list to remove previews.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_previews: Option<Vec<UpdateMessagesRequestLinkPreviewsItem>>,
}

impl UpdateMessagesRequest {
    pub fn builder() -> UpdateMessagesRequestBuilder {
        <UpdateMessagesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMessagesRequestBuilder {
    attachments: Option<Vec<UpdateMessagesRequestAttachmentsItem>>,
    content: Option<String>,
    gifs: Option<Vec<UpdateMessagesRequestGifsItem>>,
    link_previews: Option<Vec<UpdateMessagesRequestLinkPreviewsItem>>,
}

impl UpdateMessagesRequestBuilder {
    pub fn attachments(mut self, value: Vec<UpdateMessagesRequestAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn gifs(mut self, value: Vec<UpdateMessagesRequestGifsItem>) -> Self {
        self.gifs = Some(value);
        self
    }

    pub fn link_previews(mut self, value: Vec<UpdateMessagesRequestLinkPreviewsItem>) -> Self {
        self.link_previews = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMessagesRequest`].
    pub fn build(self) -> Result<UpdateMessagesRequest, BuildError> {
        Ok(UpdateMessagesRequest {
            attachments: self.attachments,
            content: self.content,
            gifs: self.gifs,
            link_previews: self.link_previews,
        })
    }
}
