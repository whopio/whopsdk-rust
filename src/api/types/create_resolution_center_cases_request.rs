pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateResolutionCenterCasesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<CreateResolutionCenterCasesRequestAttachmentsItem>>,
    /// The customer's explanation.
    #[serde(default)]
    pub message: String,
    /// What went wrong. Uses the same vocabulary as `/disputes`.
    pub reason: CreateResolutionCenterCasesRequestReason,
    /// The payment to open the case against (`pay_` tag).
    #[serde(default)]
    pub receipt_id: String,
}

impl CreateResolutionCenterCasesRequest {
    pub fn builder() -> CreateResolutionCenterCasesRequestBuilder {
        <CreateResolutionCenterCasesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateResolutionCenterCasesRequestBuilder {
    attachments: Option<Vec<CreateResolutionCenterCasesRequestAttachmentsItem>>,
    message: Option<String>,
    reason: Option<CreateResolutionCenterCasesRequestReason>,
    receipt_id: Option<String>,
}

impl CreateResolutionCenterCasesRequestBuilder {
    pub fn attachments(
        mut self,
        value: Vec<CreateResolutionCenterCasesRequestAttachmentsItem>,
    ) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn reason(mut self, value: CreateResolutionCenterCasesRequestReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn receipt_id(mut self, value: impl Into<String>) -> Self {
        self.receipt_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateResolutionCenterCasesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](CreateResolutionCenterCasesRequestBuilder::message)
    /// - [`reason`](CreateResolutionCenterCasesRequestBuilder::reason)
    /// - [`receipt_id`](CreateResolutionCenterCasesRequestBuilder::receipt_id)
    pub fn build(self) -> Result<CreateResolutionCenterCasesRequest, BuildError> {
        Ok(CreateResolutionCenterCasesRequest {
            attachments: self.attachments,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            receipt_id: self
                .receipt_id
                .ok_or_else(|| BuildError::missing_field("receipt_id"))?,
        })
    }
}
