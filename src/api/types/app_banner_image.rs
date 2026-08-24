pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppBannerImage {
    /// Banner image URL, taken from the app's product listing.
    #[serde(default)]
    pub url: String,
}

impl AppBannerImage {
    pub fn builder() -> AppBannerImageBuilder {
        <AppBannerImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppBannerImageBuilder {
    url: Option<String>,
}

impl AppBannerImageBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppBannerImage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](AppBannerImageBuilder::url)
    pub fn build(self) -> Result<AppBannerImage, BuildError> {
        Ok(AppBannerImage {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
