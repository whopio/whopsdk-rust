pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DenyResolutionCenterCasesRequest {
    /// Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<DenyResolutionCenterCasesRequestAttachmentsItem>>,
    /// Why the claim is being denied. Shown to the customer.
    #[serde(default)]
    pub message: String,
}

impl DenyResolutionCenterCasesRequest {
    pub fn builder() -> DenyResolutionCenterCasesRequestBuilder {
        <DenyResolutionCenterCasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DenyResolutionCenterCasesRequestBuilder {
    attachments: Option<Vec<DenyResolutionCenterCasesRequestAttachmentsItem>>,
    message: Option<String>,
}

impl DenyResolutionCenterCasesRequestBuilder {
    pub fn attachments(
        mut self,
        value: Vec<DenyResolutionCenterCasesRequestAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DenyResolutionCenterCasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](DenyResolutionCenterCasesRequestBuilder::message)
    pub fn build(self) -> Result<DenyResolutionCenterCasesRequest, BuildError> {
        Ok(DenyResolutionCenterCasesRequest {
            attachments: self.attachments,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
