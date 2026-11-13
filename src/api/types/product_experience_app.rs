pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductExperienceApp {
    /// App icon. Falls back to the default app icon when none is uploaded.
    #[serde(default)]
    pub icon: AppIcon,
    /// App ID, prefixed `app_`.
    #[serde(default)]
    pub id: String,
    /// App display name.
    #[serde(default)]
    pub name: String,
}

impl ProductExperienceApp {
    pub fn builder() -> ProductExperienceAppBuilder {
        <ProductExperienceAppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductExperienceAppBuilder {
    icon: Option<AppIcon>,
    id: Option<String>,
    name: Option<String>,
}

impl ProductExperienceAppBuilder {
    pub fn icon(mut self, value: AppIcon) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProductExperienceApp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`icon`](ProductExperienceAppBuilder::icon)
    /// - [`id`](ProductExperienceAppBuilder::id)
    /// - [`name`](ProductExperienceAppBuilder::name)
    pub fn build(self) -> Result<ProductExperienceApp, BuildError> {
        Ok(ProductExperienceApp {
            icon: self.icon.ok_or_else(|| BuildError::missing_field("icon"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
