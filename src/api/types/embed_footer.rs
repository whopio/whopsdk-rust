pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmbedFooter {
    /// Small image shown beside the footer text. Null when the footer has no icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<EmbedImage>,
    #[serde(default)]
    pub text: String,
}

impl EmbedFooter {
    pub fn builder() -> EmbedFooterBuilder {
        <EmbedFooterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmbedFooterBuilder {
    icon: Option<EmbedImage>,
    text: Option<String>,
}

impl EmbedFooterBuilder {
    pub fn icon(mut self, value: EmbedImage) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmbedFooter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](EmbedFooterBuilder::text)
    pub fn build(self) -> Result<EmbedFooter, BuildError> {
        Ok(EmbedFooter {
            icon: self.icon,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
