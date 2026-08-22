pub use crate::prelude::*;

/// The company whose token balance this transaction affects.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompanyTokenTransactionCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The slug/route of the company on the Whop site.
    #[serde(default)]
    pub route: String,
    /// The written name of the company.
    #[serde(default)]
    pub title: String,
}

impl CompanyTokenTransactionCompany {
    pub fn builder() -> CompanyTokenTransactionCompanyBuilder {
        <CompanyTokenTransactionCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyTokenTransactionCompanyBuilder {
    id: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl CompanyTokenTransactionCompanyBuilder {
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

    /// Consumes the builder and constructs a [`CompanyTokenTransactionCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompanyTokenTransactionCompanyBuilder::id)
    /// - [`route`](CompanyTokenTransactionCompanyBuilder::route)
    /// - [`title`](CompanyTokenTransactionCompanyBuilder::title)
    pub fn build(self) -> Result<CompanyTokenTransactionCompany, BuildError> {
        Ok(CompanyTokenTransactionCompany {
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
