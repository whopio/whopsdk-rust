pub use crate::prelude::*;

/// The company's logo.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompanyLogo {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CompanyLogo {
    pub fn builder() -> CompanyLogoBuilder {
        <CompanyLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyLogoBuilder {
    url: Option<String>,
}

impl CompanyLogoBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompanyLogo`].
    pub fn build(self) -> Result<CompanyLogo, BuildError> {
        Ok(CompanyLogo { url: self.url })
    }
}
