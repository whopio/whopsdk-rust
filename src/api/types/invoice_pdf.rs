pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicePdf {
    /// When the link stops working, as an ISO 8601 timestamp. Generate a new PDF for a fresh link.
    #[serde(default)]
    pub expires_at: String,
    /// A short-lived link to download the invoice as a PDF. Anyone holding the link can download the file until it expires.
    #[serde(default)]
    pub url: String,
}

impl InvoicePdf {
    pub fn builder() -> InvoicePdfBuilder {
        <InvoicePdfBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicePdfBuilder {
    expires_at: Option<String>,
    url: Option<String>,
}

impl InvoicePdfBuilder {
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicePdf`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](InvoicePdfBuilder::expires_at)
    /// - [`url`](InvoicePdfBuilder::url)
    pub fn build(self) -> Result<InvoicePdf, BuildError> {
        Ok(InvoicePdf {
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
