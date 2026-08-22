pub use crate::prelude::*;

/// Correspondence with the customer, or proof they used the product.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDisputesRequestEvidenceCustomerCommunicationAttachment {
    /// The ID returned by a direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The ID of an already-uploaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateDisputesRequestEvidenceCustomerCommunicationAttachment {
    pub fn builder() -> UpdateDisputesRequestEvidenceCustomerCommunicationAttachmentBuilder {
        <UpdateDisputesRequestEvidenceCustomerCommunicationAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDisputesRequestEvidenceCustomerCommunicationAttachmentBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl UpdateDisputesRequestEvidenceCustomerCommunicationAttachmentBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateDisputesRequestEvidenceCustomerCommunicationAttachment`].
    pub fn build(
        self,
    ) -> Result<UpdateDisputesRequestEvidenceCustomerCommunicationAttachment, BuildError> {
        Ok(
            UpdateDisputesRequestEvidenceCustomerCommunicationAttachment {
                direct_upload_id: self.direct_upload_id,
                id: self.id,
            },
        )
    }
}
