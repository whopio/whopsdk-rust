pub use crate::prelude::*;

/// A file upload containing the company's cancellation policy document.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEvidenceDisputeRequestCancellationPolicyAttachment {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateEvidenceDisputeRequestCancellationPolicyAttachment {
    pub fn builder() -> UpdateEvidenceDisputeRequestCancellationPolicyAttachmentBuilder {
        <UpdateEvidenceDisputeRequestCancellationPolicyAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEvidenceDisputeRequestCancellationPolicyAttachmentBuilder {
    id: Option<String>,
}

impl UpdateEvidenceDisputeRequestCancellationPolicyAttachmentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEvidenceDisputeRequestCancellationPolicyAttachment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateEvidenceDisputeRequestCancellationPolicyAttachmentBuilder::id)
    pub fn build(
        self,
    ) -> Result<UpdateEvidenceDisputeRequestCancellationPolicyAttachment, BuildError> {
        Ok(UpdateEvidenceDisputeRequestCancellationPolicyAttachment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
