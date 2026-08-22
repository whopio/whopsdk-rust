pub use crate::prelude::*;

/// A file upload containing evidence of customer communication. Must be a JPEG, PNG, GIF, or PDF.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEvidenceDisputeRequestCustomerCommunicationAttachment {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateEvidenceDisputeRequestCustomerCommunicationAttachment {
    pub fn builder() -> UpdateEvidenceDisputeRequestCustomerCommunicationAttachmentBuilder {
        <UpdateEvidenceDisputeRequestCustomerCommunicationAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEvidenceDisputeRequestCustomerCommunicationAttachmentBuilder {
    id: Option<String>,
}

impl UpdateEvidenceDisputeRequestCustomerCommunicationAttachmentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEvidenceDisputeRequestCustomerCommunicationAttachment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateEvidenceDisputeRequestCustomerCommunicationAttachmentBuilder::id)
    pub fn build(
        self,
    ) -> Result<UpdateEvidenceDisputeRequestCustomerCommunicationAttachment, BuildError> {
        Ok(
            UpdateEvidenceDisputeRequestCustomerCommunicationAttachment {
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            },
        )
    }
}
