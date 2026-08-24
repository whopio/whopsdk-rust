pub use crate::prelude::*;

/// The company's logo.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompanyListItemLogo {
    /// A pre-optimized URL for rendering this attachment on the client. This should be used for displaying attachments in apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CompanyListItemLogo {
    pub fn builder() -> CompanyListItemLogoBuilder {
        <CompanyListItemLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyListItemLogoBuilder {
    url: Option<String>,
}

impl CompanyListItemLogoBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompanyListItemLogo`].
    pub fn build(self) -> Result<CompanyListItemLogo, BuildError> {
        Ok(CompanyListItemLogo { url: self.url })
    }
}
