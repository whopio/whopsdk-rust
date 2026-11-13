pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVerificationsResponseOwnersItemRequiredDocumentsItem {
    /// Document slot key, such as `id_card_front`, `id_card_back`, or `selfie`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// Why the previous submission was rejected, when the provider requested new documents or declined the verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
    /// `pending_upload` until the document has been relayed for review; `submitted` afterwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateVerificationsResponseOwnersItemRequiredDocumentsItemStatus>,
}

impl UpdateVerificationsResponseOwnersItemRequiredDocumentsItem {
    pub fn builder() -> UpdateVerificationsResponseOwnersItemRequiredDocumentsItemBuilder {
        <UpdateVerificationsResponseOwnersItemRequiredDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsResponseOwnersItemRequiredDocumentsItemBuilder {
    document: Option<String>,
    rejection_reason: Option<String>,
    status: Option<UpdateVerificationsResponseOwnersItemRequiredDocumentsItemStatus>,
}

impl UpdateVerificationsResponseOwnersItemRequiredDocumentsItemBuilder {
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
        value: UpdateVerificationsResponseOwnersItemRequiredDocumentsItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateVerificationsResponseOwnersItemRequiredDocumentsItem`].
    pub fn build(
        self,
    ) -> Result<UpdateVerificationsResponseOwnersItemRequiredDocumentsItem, BuildError> {
        Ok(UpdateVerificationsResponseOwnersItemRequiredDocumentsItem {
            document: self.document,
            rejection_reason: self.rejection_reason,
            status: self.status,
        })
    }
}
