pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainWebsiteApp {
    /// The production base URL where the app is hosted. `null` if no base URL is configured or the domain isn't yours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// App ID, prefixed `app_`.
    #[serde(default)]
    pub id: String,
}

impl DomainWebsiteApp {
    pub fn builder() -> DomainWebsiteAppBuilder {
        <DomainWebsiteAppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWebsiteAppBuilder {
    base_url: Option<String>,
    id: Option<String>,
}

impl DomainWebsiteAppBuilder {
    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainWebsiteApp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DomainWebsiteAppBuilder::id)
    pub fn build(self) -> Result<DomainWebsiteApp, BuildError> {
        Ok(DomainWebsiteApp {
            base_url: self.base_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
