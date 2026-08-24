pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AcceptResolutionCenterCasesRequest {
    /// Up to 3 evidence files, by existing file `id` or `direct_upload_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AcceptResolutionCenterCasesRequestAttachmentsItem>>,
    /// An optional note to the customer, recorded on the case timeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Whether to also terminate the customer's membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_membership: Option<bool>,
}

impl AcceptResolutionCenterCasesRequest {
    pub fn builder() -> AcceptResolutionCenterCasesRequestBuilder {
        <AcceptResolutionCenterCasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AcceptResolutionCenterCasesRequestBuilder {
    attachments: Option<Vec<AcceptResolutionCenterCasesRequestAttachmentsItem>>,
    message: Option<String>,
    terminate_membership: Option<bool>,
}

impl AcceptResolutionCenterCasesRequestBuilder {
    pub fn attachments(
        mut self,
        value: Vec<AcceptResolutionCenterCasesRequestAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn terminate_membership(mut self, value: bool) -> Self {
        self.terminate_membership = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AcceptResolutionCenterCasesRequest`].
    pub fn build(self) -> Result<AcceptResolutionCenterCasesRequest, BuildError> {
        Ok(AcceptResolutionCenterCasesRequest {
            attachments: self.attachments,
            message: self.message,
            terminate_membership: self.terminate_membership,
        })
    }
}
