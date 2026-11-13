pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAutomationsRequest {
    /// Account ID, prefixed `biz_`. Required for user credentials; otherwise defaults to the credential's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The automation's name, shown on the dashboard. Up to 255 characters.
    #[serde(default)]
    pub name: String,
    /// The webhook event that enrolls the member it's about, such as `payment.succeeded`. `null` for an automation that only `POST /automations/{id}/trigger` enrolls into. Fixed once the automation has been published.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_event: Option<CreateAutomationsRequestTriggerEvent>,
}

impl CreateAutomationsRequest {
    pub fn builder() -> CreateAutomationsRequestBuilder {
        <CreateAutomationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAutomationsRequestBuilder {
    account_id: Option<String>,
    name: Option<String>,
    trigger_event: Option<CreateAutomationsRequestTriggerEvent>,
}

impl CreateAutomationsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn trigger_event(mut self, value: CreateAutomationsRequestTriggerEvent) -> Self {
        self.trigger_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAutomationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateAutomationsRequestBuilder::name)
    pub fn build(self) -> Result<CreateAutomationsRequest, BuildError> {
        Ok(CreateAutomationsRequest {
            account_id: self.account_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            trigger_event: self.trigger_event,
        })
    }
}
