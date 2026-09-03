pub use crate::prelude::*;

/// The company that the dispute was filed against.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeLegacyCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The written name of the company.
    #[serde(default)]
    pub title: String,
}

impl DisputeLegacyCompany {
    pub fn builder() -> DisputeLegacyCompanyBuilder {
        <DisputeLegacyCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyCompanyBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl DisputeLegacyCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacyCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeLegacyCompanyBuilder::id)
    /// - [`title`](DisputeLegacyCompanyBuilder::title)
    pub fn build(self) -> Result<DisputeLegacyCompany, BuildError> {
        Ok(DisputeLegacyCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
