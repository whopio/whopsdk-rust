pub use crate::prelude::*;

/// The cancellation policy document.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDisputesRequestEvidenceCancellationPolicyAttachment {
    /// The ID returned by a direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The ID of an already-uploaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateDisputesRequestEvidenceCancellationPolicyAttachment {
    pub fn builder() -> UpdateDisputesRequestEvidenceCancellationPolicyAttachmentBuilder {
        <UpdateDisputesRequestEvidenceCancellationPolicyAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDisputesRequestEvidenceCancellationPolicyAttachmentBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl UpdateDisputesRequestEvidenceCancellationPolicyAttachmentBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateDisputesRequestEvidenceCancellationPolicyAttachment`].
    pub fn build(
        self,
    ) -> Result<UpdateDisputesRequestEvidenceCancellationPolicyAttachment, BuildError> {
        Ok(UpdateDisputesRequestEvidenceCancellationPolicyAttachment {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
