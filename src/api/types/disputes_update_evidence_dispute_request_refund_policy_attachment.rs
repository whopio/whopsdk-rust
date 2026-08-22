pub use crate::prelude::*;

/// A file upload containing the company's refund policy document.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEvidenceDisputeRequestRefundPolicyAttachment {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateEvidenceDisputeRequestRefundPolicyAttachment {
    pub fn builder() -> UpdateEvidenceDisputeRequestRefundPolicyAttachmentBuilder {
        <UpdateEvidenceDisputeRequestRefundPolicyAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEvidenceDisputeRequestRefundPolicyAttachmentBuilder {
    id: Option<String>,
}

impl UpdateEvidenceDisputeRequestRefundPolicyAttachmentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEvidenceDisputeRequestRefundPolicyAttachment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateEvidenceDisputeRequestRefundPolicyAttachmentBuilder::id)
    pub fn build(self) -> Result<UpdateEvidenceDisputeRequestRefundPolicyAttachment, BuildError> {
        Ok(UpdateEvidenceDisputeRequestRefundPolicyAttachment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
