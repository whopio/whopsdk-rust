pub use crate::prelude::*;

/// The custom logo image for this experience. Null if no custom logo has been uploaded.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperienceListItemImage {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ExperienceListItemImage {
    pub fn builder() -> ExperienceListItemImageBuilder {
        <ExperienceListItemImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperienceListItemImageBuilder {
    url: Option<String>,
}

impl ExperienceListItemImageBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExperienceListItemImage`].
    pub fn build(self) -> Result<ExperienceListItemImage, BuildError> {
        Ok(ExperienceListItemImage { url: self.url })
    }
}
