pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppealResolutionCenterCasesRequest {
    /// Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AppealResolutionCenterCasesRequestAttachmentsItem>>,
    /// Why you are appealing the decision.
    #[serde(default)]
    pub message: String,
}

impl AppealResolutionCenterCasesRequest {
    pub fn builder() -> AppealResolutionCenterCasesRequestBuilder {
        <AppealResolutionCenterCasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppealResolutionCenterCasesRequestBuilder {
    attachments: Option<Vec<AppealResolutionCenterCasesRequestAttachmentsItem>>,
    message: Option<String>,
}

impl AppealResolutionCenterCasesRequestBuilder {
    pub fn attachments(
        mut self,
        value: Vec<AppealResolutionCenterCasesRequestAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppealResolutionCenterCasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](AppealResolutionCenterCasesRequestBuilder::message)
    pub fn build(self) -> Result<AppealResolutionCenterCasesRequest, BuildError> {
        Ok(AppealResolutionCenterCasesRequest {
            attachments: self.attachments,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
