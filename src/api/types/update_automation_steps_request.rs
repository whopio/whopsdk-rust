pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAutomationStepsRequest {
    /// What the step does: `wait` holds the member for `config.delay_seconds`, `email` emails them, and `message` sends them a message from a team member. A draft step can switch between `email` and `message`, which clears `config.sender_id`. A `wait` can't become a sending step, or the other way round.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<UpdateAutomationStepsRequestActionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<AutomationStepConfig>,
    /// An `email` or `message` step's code: a TSX module whose default export gets the member and returns `message.send()`, `message.skip()` or `message.exit()` from `whop:message`. An `email` step sends its `subject`, its `react` element as the body and its `text` as the plain-text part. A `message` step sends its `text`. The code can import `react` and `@react-email/components`. It reads the account through the `whop` client from `whop:message`, which can only read. That client holds every account-level `:read` permission that both the admin role and Whop AI hold, except `payout:account:read`, as listed for previewing a step. Setting it, or previewing the step, needs every one of those permissions. Up to 64,000 characters. `null` on a `wait`, and until it is written.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
}

impl UpdateAutomationStepsRequest {
    pub fn builder() -> UpdateAutomationStepsRequestBuilder {
        <UpdateAutomationStepsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAutomationStepsRequestBuilder {
    action_type: Option<UpdateAutomationStepsRequestActionType>,
    config: Option<AutomationStepConfig>,
    source_code: Option<String>,
}

impl UpdateAutomationStepsRequestBuilder {
    pub fn action_type(mut self, value: UpdateAutomationStepsRequestActionType) -> Self {
        self.action_type = Some(value);
        self
    }

    pub fn config(mut self, value: AutomationStepConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn source_code(mut self, value: impl Into<String>) -> Self {
        self.source_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAutomationStepsRequest`].
    pub fn build(self) -> Result<UpdateAutomationStepsRequest, BuildError> {
        Ok(UpdateAutomationStepsRequest {
            action_type: self.action_type,
            config: self.config,
            source_code: self.source_code,
        })
    }
}
