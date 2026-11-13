pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateVerificationsResponseOwnersItemRequiredDocumentsItem {
    /// Document slot key, such as `id_card_front`, `id_card_back`, or `selfie`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// Why the previous submission was rejected, when the provider requested new documents or declined the verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
    /// `pending_upload` until the document has been relayed for review; `submitted` afterwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateVerificationsResponseOwnersItemRequiredDocumentsItemStatus>,
}

impl CreateVerificationsResponseOwnersItemRequiredDocumentsItem {
    pub fn builder() -> CreateVerificationsResponseOwnersItemRequiredDocumentsItemBuilder {
        <CreateVerificationsResponseOwnersItemRequiredDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVerificationsResponseOwnersItemRequiredDocumentsItemBuilder {
    document: Option<String>,
    rejection_reason: Option<String>,
    status: Option<CreateVerificationsResponseOwnersItemRequiredDocumentsItemStatus>,
}

impl CreateVerificationsResponseOwnersItemRequiredDocumentsItemBuilder {
    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn rejection_reason(mut self, value: impl Into<String>) -> Self {
        self.rejection_reason = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: CreateVerificationsResponseOwnersItemRequiredDocumentsItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateVerificationsResponseOwnersItemRequiredDocumentsItem`].
    pub fn build(
        self,
    ) -> Result<CreateVerificationsResponseOwnersItemRequiredDocumentsItem, BuildError> {
        Ok(CreateVerificationsResponseOwnersItemRequiredDocumentsItem {
            document: self.document,
            rejection_reason: self.rejection_reason,
            status: self.status,
        })
    }
}
