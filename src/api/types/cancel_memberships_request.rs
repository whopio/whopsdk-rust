pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelMembershipsRequest {
    /// `true` stops auto-renewal and keeps access until the current billing period ends. Omit or `false` revokes access immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    /// The ID of the feedback option the customer picked as their reason for canceling, from the account's `feedback_options` preference. The membership records the option's label as the customer saw it and its category as `cancel_option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_option_id: Option<String>,
    /// Free-form note recording why the membership was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CancelMembershipsRequest {
    pub fn builder() -> CancelMembershipsRequestBuilder {
        <CancelMembershipsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelMembershipsRequestBuilder {
    cancel_at_period_end: Option<bool>,
    feedback_option_id: Option<String>,
    reason: Option<String>,
}

impl CancelMembershipsRequestBuilder {
    pub fn cancel_at_period_end(mut self, value: bool) -> Self {
        self.cancel_at_period_end = Some(value);
        self
    }

    pub fn feedback_option_id(mut self, value: impl Into<String>) -> Self {
        self.feedback_option_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CancelMembershipsRequest`].
    pub fn build(self) -> Result<CancelMembershipsRequest, BuildError> {
        Ok(CancelMembershipsRequest {
            cancel_at_period_end: self.cancel_at_period_end,
            feedback_option_id: self.feedback_option_id,
            reason: self.reason,
        })
    }
}
