pub use crate::prelude::*;

/// The company involved in this resolution case. Null if the company no longer exists.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolutionCenterCaseLegacyCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl ResolutionCenterCaseLegacyCompany {
    pub fn builder() -> ResolutionCenterCaseLegacyCompanyBuilder {
        <ResolutionCenterCaseLegacyCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCaseLegacyCompanyBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl ResolutionCenterCaseLegacyCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResolutionCenterCaseLegacyCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ResolutionCenterCaseLegacyCompanyBuilder::id)
    /// - [`title`](ResolutionCenterCaseLegacyCompanyBuilder::title)
    pub fn build(self) -> Result<ResolutionCenterCaseLegacyCompany, BuildError> {
        Ok(ResolutionCenterCaseLegacyCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
