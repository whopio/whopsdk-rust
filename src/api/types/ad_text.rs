pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdText {
    /// Always `null`. Translations are no longer available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The text shown to viewers.
    #[serde(default)]
    pub text: String,
}

impl AdText {
    pub fn builder() -> AdTextBuilder {
        <AdTextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdTextBuilder {
    language: Option<String>,
    text: Option<String>,
}

impl AdTextBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdText`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](AdTextBuilder::text)
    pub fn build(self) -> Result<AdText, BuildError> {
        Ok(AdText {
            language: self.language,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
