pub use crate::prelude::*;

/// The company that issued this invoice.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
}

impl InvoiceCompany {
    pub fn builder() -> InvoiceCompanyBuilder {
        <InvoiceCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceCompanyBuilder {
    id: Option<String>,
}

impl InvoiceCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoiceCompanyBuilder::id)
    pub fn build(self) -> Result<InvoiceCompany, BuildError> {
        Ok(InvoiceCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
