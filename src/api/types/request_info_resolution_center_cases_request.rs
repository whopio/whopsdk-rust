pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestInfoResolutionCenterCasesRequest {
    /// Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<RequestInfoResolutionCenterCasesRequestAttachmentsItem>>,
    /// What you need from the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl RequestInfoResolutionCenterCasesRequest {
    pub fn builder() -> RequestInfoResolutionCenterCasesRequestBuilder {
        <RequestInfoResolutionCenterCasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestInfoResolutionCenterCasesRequestBuilder {
    attachments: Option<Vec<RequestInfoResolutionCenterCasesRequestAttachmentsItem>>,
    message: Option<String>,
}

impl RequestInfoResolutionCenterCasesRequestBuilder {
    pub fn attachments(
        mut self,
        value: Vec<RequestInfoResolutionCenterCasesRequestAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RequestInfoResolutionCenterCasesRequest`].
    pub fn build(self) -> Result<RequestInfoResolutionCenterCasesRequest, BuildError> {
        Ok(RequestInfoResolutionCenterCasesRequest {
            attachments: self.attachments,
            message: self.message,
        })
    }
}
