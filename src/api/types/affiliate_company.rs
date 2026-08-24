pub use crate::prelude::*;

/// The company attached to this affiliate
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AffiliateCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The written name of the company.
    #[serde(default)]
    pub title: String,
}

impl AffiliateCompany {
    pub fn builder() -> AffiliateCompanyBuilder {
        <AffiliateCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffiliateCompanyBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl AffiliateCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AffiliateCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AffiliateCompanyBuilder::id)
    /// - [`title`](AffiliateCompanyBuilder::title)
    pub fn build(self) -> Result<AffiliateCompany, BuildError> {
        Ok(AffiliateCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
