pub use crate::prelude::*;

/// Company formation documents, each the ID of a file uploaded with Create File and not yet attached to anything else, prefixed `file_`. Send the ID once its `upload_status` is `ready`. Each file must be a PDF, JPEG, or PNG of up to 10MB. They are filed under the business's country, from `country`, `address`, or an earlier request. A document of a kind already on file is kept unless it was rejected, so a retried request does not add a second copy. The hosted session at `session_url` collects anything still missing.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVerificationsRequestBodyBusinessAddressDocuments {
    /// File ID of the articles of incorporation or organization, prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub articles_of_incorporation: Option<String>,
    /// File ID of the certificate of incorporation or formation, prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_of_incorporation: Option<String>,
}

impl UpdateVerificationsRequestBodyBusinessAddressDocuments {
    pub fn builder() -> UpdateVerificationsRequestBodyBusinessAddressDocumentsBuilder {
        <UpdateVerificationsRequestBodyBusinessAddressDocumentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsRequestBodyBusinessAddressDocumentsBuilder {
    articles_of_incorporation: Option<String>,
    certificate_of_incorporation: Option<String>,
}

impl UpdateVerificationsRequestBodyBusinessAddressDocumentsBuilder {
    pub fn articles_of_incorporation(mut self, value: impl Into<String>) -> Self {
        self.articles_of_incorporation = Some(value.into());
        self
    }

    pub fn certificate_of_incorporation(mut self, value: impl Into<String>) -> Self {
        self.certificate_of_incorporation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateVerificationsRequestBodyBusinessAddressDocuments`].
    pub fn build(
        self,
    ) -> Result<UpdateVerificationsRequestBodyBusinessAddressDocuments, BuildError> {
        Ok(UpdateVerificationsRequestBodyBusinessAddressDocuments {
            articles_of_incorporation: self.articles_of_incorporation,
            certificate_of_incorporation: self.certificate_of_incorporation,
        })
    }
}
