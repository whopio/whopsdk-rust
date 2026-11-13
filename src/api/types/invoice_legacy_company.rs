pub use crate::prelude::*;

/// The company that issued this invoice.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceLegacyCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
    /// The company's logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<InvoiceLegacyCompanyLogo>,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl InvoiceLegacyCompany {
    pub fn builder() -> InvoiceLegacyCompanyBuilder {
        <InvoiceLegacyCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceLegacyCompanyBuilder {
    id: Option<String>,
    logo: Option<InvoiceLegacyCompanyLogo>,
    title: Option<String>,
}

impl InvoiceLegacyCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo(mut self, value: InvoiceLegacyCompanyLogo) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceLegacyCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoiceLegacyCompanyBuilder::id)
    /// - [`title`](InvoiceLegacyCompanyBuilder::title)
    pub fn build(self) -> Result<InvoiceLegacyCompany, BuildError> {
        Ok(InvoiceLegacyCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo: self.logo,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
