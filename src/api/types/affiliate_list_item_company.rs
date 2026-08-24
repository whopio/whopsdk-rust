pub use crate::prelude::*;

/// The company attached to this affiliate
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AffiliateListItemCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The written name of the company.
    #[serde(default)]
    pub title: String,
}

impl AffiliateListItemCompany {
    pub fn builder() -> AffiliateListItemCompanyBuilder {
        <AffiliateListItemCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AffiliateListItemCompanyBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl AffiliateListItemCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AffiliateListItemCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AffiliateListItemCompanyBuilder::id)
    /// - [`title`](AffiliateListItemCompanyBuilder::title)
    pub fn build(self) -> Result<AffiliateListItemCompany, BuildError> {
        Ok(AffiliateListItemCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
