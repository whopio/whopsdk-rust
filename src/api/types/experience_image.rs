pub use crate::prelude::*;

/// The custom logo image for this experience. Null if no custom logo has been uploaded.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperienceImage {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ExperienceImage {
    pub fn builder() -> ExperienceImageBuilder {
        <ExperienceImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperienceImageBuilder {
    url: Option<String>,
}

impl ExperienceImageBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExperienceImage`].
    pub fn build(self) -> Result<ExperienceImage, BuildError> {
        Ok(ExperienceImage { url: self.url })
    }
}
