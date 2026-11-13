pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AutomationStep {
    /// What the step does: `wait` holds the member for `config.delay_seconds`, `email` emails them, and `message` sends them a message from a team member. A draft step can switch between `email` and `message`, which clears `config.sender_id`. A `wait` can't become a sending step, or the other way round.
    pub action_type: AutomationStepActionType,
    /// ID of the automation the step belongs to, prefixed `ato_`.
    #[serde(default)]
    pub automation_id: String,
    /// The step's settings for its `action_type`: `delay_seconds` for a `wait`, `sender_id` for an `email` or `message`. Writing it replaces the whole object.
    pub config: AutomationStepConfig,
    /// When the step was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Automation step ID, prefixed `atos_`. A step gets a new ID in every version. Its `key` stays the same.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub issues: Vec<String>,
    /// Identifies the same step across versions. A new step gets a key no step of the automation has used.
    #[serde(default)]
    pub key: i64,
    /// ID of the step this one runs after, prefixed `atos_`. `null` when it runs first, directly after the member enrolls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_step_id: Option<String>,
    /// The sender `config.sender_id` names. `null` on a `wait`, and when the step sends from the default sender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<AutomationStepSender>,
    /// An `email` or `message` step's code: a TSX module whose default export gets the member and returns `message.send()`, `message.skip()` or `message.exit()` from `whop:message`. An `email` step sends its `subject`, its `react` element as the body and its `text` as the plain-text part. A `message` step sends its `text`. The code can import `react` and `@react-email/components`. It reads the account through the `whop` client from `whop:message`, which can only read. That client holds every account-level `:read` permission that both the admin role and Whop AI hold, except `payout:account:read`, as listed for previewing a step. Setting it, or previewing the step, needs every one of those permissions. Up to 64,000 characters. `null` on a `wait`, and until it is written.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code: Option<String>,
    /// When the step was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The automation version this step is part of. Only the draft version's steps can change.
    #[serde(default)]
    pub version: i64,
    /// Members on this step's version who are waiting for it to run.
    #[serde(default)]
    pub waiting_enrollments_count: i64,
}

impl AutomationStep {
    pub fn builder() -> AutomationStepBuilder {
        <AutomationStepBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationStepBuilder {
    action_type: Option<AutomationStepActionType>,
    automation_id: Option<String>,
    config: Option<AutomationStepConfig>,
    created_at: Option<String>,
    id: Option<String>,
    issues: Option<Vec<String>>,
    key: Option<i64>,
    previous_step_id: Option<String>,
    sender: Option<AutomationStepSender>,
    source_code: Option<String>,
    updated_at: Option<String>,
    version: Option<i64>,
    waiting_enrollments_count: Option<i64>,
}

impl AutomationStepBuilder {
    pub fn action_type(mut self, value: AutomationStepActionType) -> Self {
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

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn issues(mut self, value: Vec<String>) -> Self {
        self.issues = Some(value);
        self
    }

    pub fn key(mut self, value: i64) -> Self {
        self.key = Some(value);
        self
    }

    pub fn previous_step_id(mut self, value: impl Into<String>) -> Self {
        self.previous_step_id = Some(value.into());
        self
    }

    pub fn sender(mut self, value: AutomationStepSender) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn source_code(mut self, value: impl Into<String>) -> Self {
        self.source_code = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn waiting_enrollments_count(mut self, value: i64) -> Self {
        self.waiting_enrollments_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationStep`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action_type`](AutomationStepBuilder::action_type)
    /// - [`automation_id`](AutomationStepBuilder::automation_id)
    /// - [`config`](AutomationStepBuilder::config)
    /// - [`created_at`](AutomationStepBuilder::created_at)
    /// - [`id`](AutomationStepBuilder::id)
    /// - [`issues`](AutomationStepBuilder::issues)
    /// - [`key`](AutomationStepBuilder::key)
    /// - [`updated_at`](AutomationStepBuilder::updated_at)
    /// - [`version`](AutomationStepBuilder::version)
    /// - [`waiting_enrollments_count`](AutomationStepBuilder::waiting_enrollments_count)
    pub fn build(self) -> Result<AutomationStep, BuildError> {
        Ok(AutomationStep {
            action_type: self
                .action_type
                .ok_or_else(|| BuildError::missing_field("action_type"))?,
            automation_id: self
                .automation_id
                .ok_or_else(|| BuildError::missing_field("automation_id"))?,
            config: self
                .config
                .ok_or_else(|| BuildError::missing_field("config"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issues: self
                .issues
                .ok_or_else(|| BuildError::missing_field("issues"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            previous_step_id: self.previous_step_id,
            sender: self.sender,
            source_code: self.source_code,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            waiting_enrollments_count: self
                .waiting_enrollments_count
                .ok_or_else(|| BuildError::missing_field("waiting_enrollments_count"))?,
        })
    }
}
