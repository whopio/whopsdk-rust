pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppDomain {
    /// Normalized hostname assigned to this app.
    #[serde(default)]
    pub domain: String,
    /// Domain ID, prefixed `dom_`.
    #[serde(default)]
    pub id: String,
    /// Domain lifecycle status, matching the domain resource.
    pub status: AppDomainStatus,
}

impl AppDomain {
    pub fn builder() -> AppDomainBuilder {
        <AppDomainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppDomainBuilder {
    domain: Option<String>,
    id: Option<String>,
    status: Option<AppDomainStatus>,
}

impl AppDomainBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: AppDomainStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppDomain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](AppDomainBuilder::domain)
    /// - [`id`](AppDomainBuilder::id)
    /// - [`status`](AppDomainBuilder::status)
    pub fn build(self) -> Result<AppDomain, BuildError> {
        Ok(AppDomain {
            domain: self
                .domain
                .ok_or_else(|| BuildError::missing_field("domain"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
