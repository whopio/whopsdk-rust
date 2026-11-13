pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAutomationStepsRequest {
    /// What the step does: `wait` holds the member for `config.delay_seconds`, `email` emails them, and `message` sends them a message from a team member. A draft step can switch between `email` and `message`, which clears `config.sender_id`. A `wait` can't become a sending step, or the other way round.
    pub action_type: CreateAutomationStepsRequestActionType,
    /// ID of the automation to add the step to, prefixed `ato_`.
    #[serde(default)]
    pub automation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<AutomationStepConfig>,
    /// ID of the draft step the new one runs after, prefixed `atos_`. Leave it out to make the new step run first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_step_id: Option<String>,
    /// An `email` or `message` step's code: a TSX module whose default export gets the member and returns `message.send()`, `message.skip()` or `message.exit()` from `whop:message`. An `email` step sends its `subject`, its `react` element as the body and its `text` as the plain-text part. A `message` step sends its `text`. The code can import `react` and `@react-email/components`. It reads the account through the `whop` client from `whop:message`, which can only read. That client holds every account-level `:read` permission that both the admin role and Whop AI hold, except `payout:account:read`, as listed for previewing a step. Setting it, or previewing the step, needs every one of those permissions. Up to 64,000 characters. `null` on a `wait`, and until it is written.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
}

impl CreateAutomationStepsRequest {
    pub fn builder() -> CreateAutomationStepsRequestBuilder {
        <CreateAutomationStepsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAutomationStepsRequestBuilder {
    action_type: Option<CreateAutomationStepsRequestActionType>,
    automation_id: Option<String>,
    config: Option<AutomationStepConfig>,
    previous_step_id: Option<String>,
    source_code: Option<String>,
}

impl CreateAutomationStepsRequestBuilder {
    pub fn action_type(mut self, value: CreateAutomationStepsRequestActionType) -> Self {
        self.action_type = Some(value);
        self
    }

    pub fn automation_id(mut self, value: impl Into<String>) -> Self {
        self.automation_id = Some(value.into());
        self
    }

    pub fn config(mut self, value: AutomationStepConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn previous_step_id(mut self, value: impl Into<String>) -> Self {
        self.previous_step_id = Some(value.into());
        self
    }

    pub fn source_code(mut self, value: impl Into<String>) -> Self {
        self.source_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAutomationStepsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action_type`](CreateAutomationStepsRequestBuilder::action_type)
    /// - [`automation_id`](CreateAutomationStepsRequestBuilder::automation_id)
    pub fn build(self) -> Result<CreateAutomationStepsRequest, BuildError> {
        Ok(CreateAutomationStepsRequest {
            action_type: self
                .action_type
                .ok_or_else(|| BuildError::missing_field("action_type"))?,
            automation_id: self
                .automation_id
                .ok_or_else(|| BuildError::missing_field("automation_id"))?,
            config: self.config,
            previous_step_id: self.previous_step_id,
            source_code: self.source_code,
        })
    }
}
