pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppIcon {
    /// Icon image URL. Always present — the default app icon when none is uploaded.
    #[serde(default)]
    pub url: String,
}

impl AppIcon {
    pub fn builder() -> AppIconBuilder {
        <AppIconBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppIconBuilder {
    url: Option<String>,
}

impl AppIconBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppIcon`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](AppIconBuilder::url)
    pub fn build(self) -> Result<AppIcon, BuildError> {
        Ok(AppIcon {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
