pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainRegistrar {
    /// The registrar's IANA registrar ID, or `null` when the record omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iana_id: Option<String>,
    /// Registrar name, such as `GoDaddy.com, LLC`.
    #[serde(default)]
    pub name: String,
    /// The registrar's website, or `null` when the record omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DomainRegistrar {
    pub fn builder() -> DomainRegistrarBuilder {
        <DomainRegistrarBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainRegistrarBuilder {
    iana_id: Option<String>,
    name: Option<String>,
    url: Option<String>,
}

impl DomainRegistrarBuilder {
    pub fn iana_id(mut self, value: impl Into<String>) -> Self {
        self.iana_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainRegistrar`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainRegistrarBuilder::name)
    pub fn build(self) -> Result<DomainRegistrar, BuildError> {
        Ok(DomainRegistrar {
            iana_id: self.iana_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url,
        })
    }
}
