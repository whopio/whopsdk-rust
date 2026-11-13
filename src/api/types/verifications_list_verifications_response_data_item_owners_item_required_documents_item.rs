pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItem {
    /// Document slot key, such as `id_card_front`, `id_card_back`, or `selfie`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    /// Why the previous submission was rejected, when the provider requested new documents or declined the verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
    /// `pending_upload` until the document has been relayed for review; `submitted` afterwards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItemStatus>,
}

impl ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItem {
    pub fn builder() -> ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItemBuilder {
        <ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItemBuilder {
    document: Option<String>,
    rejection_reason: Option<String>,
    status: Option<ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItemStatus>,
}

impl ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItemBuilder {
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
        value: ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItemStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItem`].
    pub fn build(
        self,
    ) -> Result<ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItem, BuildError> {
        Ok(
            ListVerificationsResponseDataItemOwnersItemRequiredDocumentsItem {
                document: self.document,
                rejection_reason: self.rejection_reason,
                status: self.status,
            },
        )
    }
}
