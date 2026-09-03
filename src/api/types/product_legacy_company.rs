pub use crate::prelude::*;

/// The company this product belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductLegacyCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// URL slug for the account's store page, e.g. `pickaxe` in whop.com/pickaxe.
    #[serde(default)]
    pub route: String,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl ProductLegacyCompany {
    pub fn builder() -> ProductLegacyCompanyBuilder {
        <ProductLegacyCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductLegacyCompanyBuilder {
    id: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl ProductLegacyCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProductLegacyCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProductLegacyCompanyBuilder::id)
    /// - [`route`](ProductLegacyCompanyBuilder::route)
    /// - [`title`](ProductLegacyCompanyBuilder::title)
    pub fn build(self) -> Result<ProductLegacyCompany, BuildError> {
        Ok(ProductLegacyCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
