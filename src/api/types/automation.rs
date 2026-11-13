pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Automation {
    /// ID of the account the automation belongs to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Members currently in the automation.
    #[serde(default)]
    pub active_enrollments_count: i64,
    /// When the automation was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Who created the automation.
    pub created_by_actor: AutomationActor,
    /// The version that step edits change. Always `published_version` plus one, or `1` before the first publish.
    #[serde(default)]
    pub draft_version: i64,
    /// Whether the draft has edits that `POST /automations/{id}/publish` would publish. Always `true` before the first publish.
    #[serde(default)]
    pub has_unpublished_changes: bool,
    /// Automation ID, prefixed `ato_`.
    #[serde(default)]
    pub id: String,
    /// The automation's name, shown on the dashboard. Up to 255 characters.
    #[serde(default)]
    pub name: String,
    /// When the automation was last published, as an ISO 8601 timestamp. `null` until its first publish.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    /// Who last published the automation. `null` until its first publish.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_by_actor: Option<AutomationActor>,
    /// The version new members start on. `null` until the automation is first published.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_version: Option<i64>,
    #[serde(default)]
    pub secret_names: Vec<String>,
    /// `active` enrolls members and runs their steps. `closed` enrolls no one new, and members already in it finish. `stopped` enrolls no one, and its members' enrollments have ended. A new automation is `stopped` until its first publish makes it `active`.
    pub status: AutomationStatus,
    /// How many times members have enrolled in the automation, including enrollments that have ended.
    #[serde(default)]
    pub total_enrollments_count: i64,
    /// The webhook event that enrolls the member it's about, such as `payment.succeeded`. `null` for an automation that only `POST /automations/{id}/trigger` enrolls into. Fixed once the automation has been published.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_event: Option<AutomationTriggerEvent>,
    /// When the automation was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl Automation {
    pub fn builder() -> AutomationBuilder {
        <AutomationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationBuilder {
    account_id: Option<String>,
    active_enrollments_count: Option<i64>,
    created_at: Option<String>,
    created_by_actor: Option<AutomationActor>,
    draft_version: Option<i64>,
    has_unpublished_changes: Option<bool>,
    id: Option<String>,
    name: Option<String>,
    published_at: Option<String>,
    published_by_actor: Option<AutomationActor>,
    published_version: Option<i64>,
    secret_names: Option<Vec<String>>,
    status: Option<AutomationStatus>,
    total_enrollments_count: Option<i64>,
    trigger_event: Option<AutomationTriggerEvent>,
    updated_at: Option<String>,
}

impl AutomationBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn active_enrollments_count(mut self, value: i64) -> Self {
        self.active_enrollments_count = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn created_by_actor(mut self, value: AutomationActor) -> Self {
        self.created_by_actor = Some(value);
        self
    }

    pub fn draft_version(mut self, value: i64) -> Self {
        self.draft_version = Some(value);
        self
    }

    pub fn has_unpublished_changes(mut self, value: bool) -> Self {
        self.has_unpublished_changes = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn published_at(mut self, value: impl Into<String>) -> Self {
        self.published_at = Some(value.into());
        self
    }

    pub fn published_by_actor(mut self, value: AutomationActor) -> Self {
        self.published_by_actor = Some(value);
        self
    }

    pub fn published_version(mut self, value: i64) -> Self {
        self.published_version = Some(value);
        self
    }

    pub fn secret_names(mut self, value: Vec<String>) -> Self {
        self.secret_names = Some(value);
        self
    }

    pub fn status(mut self, value: AutomationStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn total_enrollments_count(mut self, value: i64) -> Self {
        self.total_enrollments_count = Some(value);
        self
    }

    pub fn trigger_event(mut self, value: AutomationTriggerEvent) -> Self {
        self.trigger_event = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Automation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AutomationBuilder::account_id)
    /// - [`active_enrollments_count`](AutomationBuilder::active_enrollments_count)
    /// - [`created_at`](AutomationBuilder::created_at)
    /// - [`created_by_actor`](AutomationBuilder::created_by_actor)
    /// - [`draft_version`](AutomationBuilder::draft_version)
    /// - [`has_unpublished_changes`](AutomationBuilder::has_unpublished_changes)
    /// - [`id`](AutomationBuilder::id)
    /// - [`name`](AutomationBuilder::name)
    /// - [`secret_names`](AutomationBuilder::secret_names)
    /// - [`status`](AutomationBuilder::status)
    /// - [`total_enrollments_count`](AutomationBuilder::total_enrollments_count)
    /// - [`updated_at`](AutomationBuilder::updated_at)
    pub fn build(self) -> Result<Automation, BuildError> {
        Ok(Automation {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            active_enrollments_count: self
                .active_enrollments_count
                .ok_or_else(|| BuildError::missing_field("active_enrollments_count"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by_actor: self
                .created_by_actor
                .ok_or_else(|| BuildError::missing_field("created_by_actor"))?,
            draft_version: self
                .draft_version
                .ok_or_else(|| BuildError::missing_field("draft_version"))?,
            has_unpublished_changes: self
                .has_unpublished_changes
                .ok_or_else(|| BuildError::missing_field("has_unpublished_changes"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            published_at: self.published_at,
            published_by_actor: self.published_by_actor,
            published_version: self.published_version,
            secret_names: self
                .secret_names
                .ok_or_else(|| BuildError::missing_field("secret_names"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            total_enrollments_count: self
                .total_enrollments_count
                .ok_or_else(|| BuildError::missing_field("total_enrollments_count"))?,
            trigger_event: self.trigger_event,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
