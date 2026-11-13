pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AutomationEnrollment {
    /// ID of the account the automation belongs to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// ID of the automation, prefixed `ato_`.
    #[serde(default)]
    pub automation_id: String,
    /// When the member enrolled, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When the enrollment ended, as an ISO 8601 timestamp. `null` while it's active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// Automation enrollment ID, prefixed `atoe_`.
    #[serde(default)]
    pub id: String,
    /// ID of the member in the automation, prefixed `mber_`.
    #[serde(default)]
    pub member_id: String,
    /// When that step runs for this member, as an ISO 8601 timestamp. `null` once the enrollment ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_run_at: Option<String>,
    /// ID of that step in the member's version, prefixed `atos_`. `null` once the enrollment ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step_id: Option<String>,
    /// The `key` of the step the member is waiting on, in their version. `null` once the enrollment ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step_key: Option<i64>,
    /// `active` while the member moves through the automation. `completed` after its last step. `canceled` when the account canceled the enrollment, or stopped or deleted the automation.
    pub status: AutomationEnrollmentStatus,
    /// The webhook event that enrolled the member. `null` when `POST /automations/{id}/trigger` enrolled them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_event: Option<AutomationEnrollmentTriggerEvent>,
    /// ID of the record the triggering event was about, such as a `pay_` payment or `mem_` membership. `null` when a trigger request enrolled them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_resource_id: Option<String>,
    /// When the enrollment last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// ID of the member's user, prefixed `user_`. `null` when the member is another business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The automation version the member is on. It changes only when a publish moves members to the new version.
    #[serde(default)]
    pub version: i64,
}

impl AutomationEnrollment {
    pub fn builder() -> AutomationEnrollmentBuilder {
        <AutomationEnrollmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationEnrollmentBuilder {
    account_id: Option<String>,
    automation_id: Option<String>,
    created_at: Option<String>,
    ended_at: Option<String>,
    id: Option<String>,
    member_id: Option<String>,
    next_run_at: Option<String>,
    next_step_id: Option<String>,
    next_step_key: Option<i64>,
    status: Option<AutomationEnrollmentStatus>,
    trigger_event: Option<AutomationEnrollmentTriggerEvent>,
    trigger_resource_id: Option<String>,
    updated_at: Option<String>,
    user_id: Option<String>,
    version: Option<i64>,
}

impl AutomationEnrollmentBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn automation_id(mut self, value: impl Into<String>) -> Self {
        self.automation_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn ended_at(mut self, value: impl Into<String>) -> Self {
        self.ended_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn next_run_at(mut self, value: impl Into<String>) -> Self {
        self.next_run_at = Some(value.into());
        self
    }

    pub fn next_step_id(mut self, value: impl Into<String>) -> Self {
        self.next_step_id = Some(value.into());
        self
    }

    pub fn next_step_key(mut self, value: i64) -> Self {
        self.next_step_key = Some(value);
        self
    }

    pub fn status(mut self, value: AutomationEnrollmentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn trigger_event(mut self, value: AutomationEnrollmentTriggerEvent) -> Self {
        self.trigger_event = Some(value);
        self
    }

    pub fn trigger_resource_id(mut self, value: impl Into<String>) -> Self {
        self.trigger_resource_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationEnrollment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AutomationEnrollmentBuilder::account_id)
    /// - [`automation_id`](AutomationEnrollmentBuilder::automation_id)
    /// - [`created_at`](AutomationEnrollmentBuilder::created_at)
    /// - [`id`](AutomationEnrollmentBuilder::id)
    /// - [`member_id`](AutomationEnrollmentBuilder::member_id)
    /// - [`status`](AutomationEnrollmentBuilder::status)
    /// - [`updated_at`](AutomationEnrollmentBuilder::updated_at)
    /// - [`version`](AutomationEnrollmentBuilder::version)
    pub fn build(self) -> Result<AutomationEnrollment, BuildError> {
        Ok(AutomationEnrollment {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            automation_id: self
                .automation_id
                .ok_or_else(|| BuildError::missing_field("automation_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            ended_at: self.ended_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            member_id: self
                .member_id
                .ok_or_else(|| BuildError::missing_field("member_id"))?,
            next_run_at: self.next_run_at,
            next_step_id: self.next_step_id,
            next_step_key: self.next_step_key,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            trigger_event: self.trigger_event,
            trigger_resource_id: self.trigger_resource_id,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user_id: self.user_id,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
