pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountCompanyFormationDocument {
    /// Document ID, prefixed `file_`.
    #[serde(default)]
    pub id: String,
    /// Human-readable document name, such as `Articles of Organization`.
    #[serde(default)]
    pub name: String,
    /// Document category: `articles_of_organization`, `operating_agreement`, `ein_letter`, `signed_ss4`, `signed_form8821`, or `mail` for postal correspondence received on the company's behalf.
    #[serde(default)]
    pub r#type: String,
    /// CDN URL for downloading the document.
    #[serde(default)]
    pub url: String,
}

impl AccountCompanyFormationDocument {
    pub fn builder() -> AccountCompanyFormationDocumentBuilder {
        <AccountCompanyFormationDocumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountCompanyFormationDocumentBuilder {
    id: Option<String>,
    name: Option<String>,
    r#type: Option<String>,
    url: Option<String>,
}

impl AccountCompanyFormationDocumentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountCompanyFormationDocument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountCompanyFormationDocumentBuilder::id)
    /// - [`name`](AccountCompanyFormationDocumentBuilder::name)
    /// - [`r#type`](AccountCompanyFormationDocumentBuilder::r#type)
    /// - [`url`](AccountCompanyFormationDocumentBuilder::url)
    pub fn build(self) -> Result<AccountCompanyFormationDocument, BuildError> {
        Ok(AccountCompanyFormationDocument {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
