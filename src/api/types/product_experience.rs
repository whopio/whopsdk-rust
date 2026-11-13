pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductExperience {
    /// App the experience runs in.
    #[serde(default)]
    pub app: ProductExperienceApp,
    /// Experience ID, prefixed `exp_`.
    #[serde(default)]
    pub id: String,
    /// Logo image URL uploaded for this experience. `null` when it has none; show the app's `icon` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Experience display name.
    #[serde(default)]
    pub name: String,
}

impl ProductExperience {
    pub fn builder() -> ProductExperienceBuilder {
        <ProductExperienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductExperienceBuilder {
    app: Option<ProductExperienceApp>,
    id: Option<String>,
    logo_url: Option<String>,
    name: Option<String>,
}

impl ProductExperienceBuilder {
    pub fn app(mut self, value: ProductExperienceApp) -> Self {
        self.app = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProductExperience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`app`](ProductExperienceBuilder::app)
    /// - [`id`](ProductExperienceBuilder::id)
    /// - [`name`](ProductExperienceBuilder::name)
    pub fn build(self) -> Result<ProductExperience, BuildError> {
        Ok(ProductExperience {
            app: self.app.ok_or_else(|| BuildError::missing_field("app"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
