pub use crate::prelude::*;

/// The refund policy document.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDisputesRequestEvidenceRefundPolicyAttachment {
    /// The ID returned by a direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The ID of an already-uploaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateDisputesRequestEvidenceRefundPolicyAttachment {
    pub fn builder() -> UpdateDisputesRequestEvidenceRefundPolicyAttachmentBuilder {
        <UpdateDisputesRequestEvidenceRefundPolicyAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDisputesRequestEvidenceRefundPolicyAttachmentBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl UpdateDisputesRequestEvidenceRefundPolicyAttachmentBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateDisputesRequestEvidenceRefundPolicyAttachment`].
    pub fn build(self) -> Result<UpdateDisputesRequestEvidenceRefundPolicyAttachment, BuildError> {
        Ok(UpdateDisputesRequestEvidenceRefundPolicyAttachment {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
