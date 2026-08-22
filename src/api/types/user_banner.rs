pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserBanner {
    /// Profile banner image URL.
    #[serde(default)]
    pub url: String,
}

impl UserBanner {
    pub fn builder() -> UserBannerBuilder {
        <UserBannerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBannerBuilder {
    url: Option<String>,
}

impl UserBannerBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserBanner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](UserBannerBuilder::url)
    pub fn build(self) -> Result<UserBanner, BuildError> {
        Ok(UserBanner {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
