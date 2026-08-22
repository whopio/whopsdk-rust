pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplyResolutionCenterCasesRequest {
    /// Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<ReplyResolutionCenterCasesRequestAttachmentsItem>>,
    /// The reply to add to the case.
    #[serde(default)]
    pub message: String,
}

impl ReplyResolutionCenterCasesRequest {
    pub fn builder() -> ReplyResolutionCenterCasesRequestBuilder {
        <ReplyResolutionCenterCasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplyResolutionCenterCasesRequestBuilder {
    attachments: Option<Vec<ReplyResolutionCenterCasesRequestAttachmentsItem>>,
    message: Option<String>,
}

impl ReplyResolutionCenterCasesRequestBuilder {
    pub fn attachments(
        mut self,
        value: Vec<ReplyResolutionCenterCasesRequestAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReplyResolutionCenterCasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ReplyResolutionCenterCasesRequestBuilder::message)
    pub fn build(self) -> Result<ReplyResolutionCenterCasesRequest, BuildError> {
        Ok(ReplyResolutionCenterCasesRequest {
            attachments: self.attachments,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
