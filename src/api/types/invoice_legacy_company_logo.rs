pub use crate::prelude::*;

/// The company's logo.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceLegacyCompanyLogo {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl InvoiceLegacyCompanyLogo {
    pub fn builder() -> InvoiceLegacyCompanyLogoBuilder {
        <InvoiceLegacyCompanyLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceLegacyCompanyLogoBuilder {
    url: Option<String>,
}

impl InvoiceLegacyCompanyLogoBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceLegacyCompanyLogo`].
    pub fn build(self) -> Result<InvoiceLegacyCompanyLogo, BuildError> {
        Ok(InvoiceLegacyCompanyLogo { url: self.url })
    }
}
