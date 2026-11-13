pub use crate::prelude::*;

/// The company's logo.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LegacyInvoiceCompanyLogo {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl LegacyInvoiceCompanyLogo {
    pub fn builder() -> LegacyInvoiceCompanyLogoBuilder {
        <LegacyInvoiceCompanyLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LegacyInvoiceCompanyLogoBuilder {
    url: Option<String>,
}

impl LegacyInvoiceCompanyLogoBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LegacyInvoiceCompanyLogo`].
    pub fn build(self) -> Result<LegacyInvoiceCompanyLogo, BuildError> {
        Ok(LegacyInvoiceCompanyLogo { url: self.url })
    }
}
