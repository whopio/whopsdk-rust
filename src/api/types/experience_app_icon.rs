pub use crate::prelude::*;

/// The icon image for this app, displayed on the app store, product pages, checkout, and as the default icon for experiences using this app.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperienceAppIcon {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ExperienceAppIcon {
    pub fn builder() -> ExperienceAppIconBuilder {
        <ExperienceAppIconBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperienceAppIconBuilder {
    url: Option<String>,
}

impl ExperienceAppIconBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExperienceAppIcon`].
    pub fn build(self) -> Result<ExperienceAppIcon, BuildError> {
        Ok(ExperienceAppIcon { url: self.url })
    }
}
