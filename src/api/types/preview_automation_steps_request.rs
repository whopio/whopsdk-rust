pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewAutomationStepsRequest {
    /// The member to preview the step for, prefixed `mber_`.
    #[serde(default)]
    pub member_id: String,
    /// ID of the record the automation's trigger event was about for this member, such as a `pay_` payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_resource_id: Option<String>,
}

impl PreviewAutomationStepsRequest {
    pub fn builder() -> PreviewAutomationStepsRequestBuilder {
        <PreviewAutomationStepsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewAutomationStepsRequestBuilder {
    member_id: Option<String>,
    trigger_resource_id: Option<String>,
}

impl PreviewAutomationStepsRequestBuilder {
    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn trigger_resource_id(mut self, value: impl Into<String>) -> Self {
        self.trigger_resource_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewAutomationStepsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`member_id`](PreviewAutomationStepsRequestBuilder::member_id)
    pub fn build(self) -> Result<PreviewAutomationStepsRequest, BuildError> {
        Ok(PreviewAutomationStepsRequest {
            member_id: self
                .member_id
                .ok_or_else(|| BuildError::missing_field("member_id"))?,
            trigger_resource_id: self.trigger_resource_id,
        })
    }
}
