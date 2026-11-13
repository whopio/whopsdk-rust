pub use crate::prelude::*;

/// The company that issued this invoice.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LegacyInvoiceCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The company's logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<LegacyInvoiceCompanyLogo>,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl LegacyInvoiceCompany {
    pub fn builder() -> LegacyInvoiceCompanyBuilder {
        <LegacyInvoiceCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LegacyInvoiceCompanyBuilder {
    id: Option<String>,
    logo: Option<LegacyInvoiceCompanyLogo>,
    title: Option<String>,
}

impl LegacyInvoiceCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo(mut self, value: LegacyInvoiceCompanyLogo) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LegacyInvoiceCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LegacyInvoiceCompanyBuilder::id)
    /// - [`title`](LegacyInvoiceCompanyBuilder::title)
    pub fn build(self) -> Result<LegacyInvoiceCompany, BuildError> {
        Ok(LegacyInvoiceCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo: self.logo,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
