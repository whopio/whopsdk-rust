pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessageLinkPreview {
    /// Page description, or null while unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Site icon URL, or null when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    /// Additional preview information, or null when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<MessageLinkFooter>,
    /// Preview image URL, or null when unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Whether the server is still generating this preview.
    #[serde(default)]
    pub processing: bool,
    /// Page title, or null while unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// URL being previewed.
    #[serde(default)]
    pub url: String,
}

impl MessageLinkPreview {
    pub fn builder() -> MessageLinkPreviewBuilder {
        <MessageLinkPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageLinkPreviewBuilder {
    description: Option<String>,
    favicon_url: Option<String>,
    footer: Option<MessageLinkFooter>,
    image_url: Option<String>,
    processing: Option<bool>,
    title: Option<String>,
    url: Option<String>,
}

impl MessageLinkPreviewBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn favicon_url(mut self, value: impl Into<String>) -> Self {
        self.favicon_url = Some(value.into());
        self
    }

    pub fn footer(mut self, value: MessageLinkFooter) -> Self {
        self.footer = Some(value);
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn processing(mut self, value: bool) -> Self {
        self.processing = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MessageLinkPreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`processing`](MessageLinkPreviewBuilder::processing)
    /// - [`url`](MessageLinkPreviewBuilder::url)
    pub fn build(self) -> Result<MessageLinkPreview, BuildError> {
        Ok(MessageLinkPreview {
            description: self.description,
            favicon_url: self.favicon_url,
            footer: self.footer,
            image_url: self.image_url,
            processing: self
                .processing
                .ok_or_else(|| BuildError::missing_field("processing"))?,
            title: self.title,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
