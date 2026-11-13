pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MessageLinkFooter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_url: String,
    #[serde(default)]
    pub title: String,
}

impl MessageLinkFooter {
    pub fn builder() -> MessageLinkFooterBuilder {
        <MessageLinkFooterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MessageLinkFooterBuilder {
    description: Option<String>,
    icon_url: Option<String>,
    title: Option<String>,
}

impl MessageLinkFooterBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MessageLinkFooter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`icon_url`](MessageLinkFooterBuilder::icon_url)
    /// - [`title`](MessageLinkFooterBuilder::title)
    pub fn build(self) -> Result<MessageLinkFooter, BuildError> {
        Ok(MessageLinkFooter {
            description: self.description,
            icon_url: self
                .icon_url
                .ok_or_else(|| BuildError::missing_field("icon_url"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
