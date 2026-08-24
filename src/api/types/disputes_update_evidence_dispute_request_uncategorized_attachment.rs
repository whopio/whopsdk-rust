pub use crate::prelude::*;

/// A file upload for evidence that does not fit into the other categories.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEvidenceDisputeRequestUncategorizedAttachment {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateEvidenceDisputeRequestUncategorizedAttachment {
    pub fn builder() -> UpdateEvidenceDisputeRequestUncategorizedAttachmentBuilder {
        <UpdateEvidenceDisputeRequestUncategorizedAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEvidenceDisputeRequestUncategorizedAttachmentBuilder {
    id: Option<String>,
}

impl UpdateEvidenceDisputeRequestUncategorizedAttachmentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEvidenceDisputeRequestUncategorizedAttachment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateEvidenceDisputeRequestUncategorizedAttachmentBuilder::id)
    pub fn build(self) -> Result<UpdateEvidenceDisputeRequestUncategorizedAttachment, BuildError> {
        Ok(UpdateEvidenceDisputeRequestUncategorizedAttachment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
