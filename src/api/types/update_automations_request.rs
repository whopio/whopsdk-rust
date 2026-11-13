pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAutomationsRequest {
    /// The automation's name, shown on the dashboard. Up to 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Secrets to set, by name, merged into the automation's: a name set to `null` is removed. Step code reads them as environment variables, and no endpoint returns their values. Names are uppercase letters, digits and underscores, starting with a letter, up to 64 characters. Up to 50 secrets, each up to 4 KB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<HashMap<String, Option<String>>>,
    /// `active` enrolls members and runs their steps. `closed` stops new enrollments and lets members already in it finish. `stopped` stops new enrollments and ends every member's enrollment. `active` and `closed` need a published version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateAutomationsRequestStatus>,
    /// The webhook event that enrolls the member it's about, such as `payment.succeeded`. `null` for an automation that only `POST /automations/{id}/trigger` enrolls into. Fixed once the automation has been published.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_event: Option<UpdateAutomationsRequestTriggerEvent>,
}

impl UpdateAutomationsRequest {
    pub fn builder() -> UpdateAutomationsRequestBuilder {
        <UpdateAutomationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAutomationsRequestBuilder {
    name: Option<String>,
    secrets: Option<HashMap<String, Option<String>>>,
    status: Option<UpdateAutomationsRequestStatus>,
    trigger_event: Option<UpdateAutomationsRequestTriggerEvent>,
}

impl UpdateAutomationsRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn secrets(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.secrets = Some(value);
        self
    }

    pub fn status(mut self, value: UpdateAutomationsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn trigger_event(mut self, value: UpdateAutomationsRequestTriggerEvent) -> Self {
        self.trigger_event = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAutomationsRequest`].
    pub fn build(self) -> Result<UpdateAutomationsRequest, BuildError> {
        Ok(UpdateAutomationsRequest {
            name: self.name,
            secrets: self.secrets,
            status: self.status,
            trigger_event: self.trigger_event,
        })
    }
}
