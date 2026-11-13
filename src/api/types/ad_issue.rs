pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdIssue {
    /// When the issue was first reported, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The issue's ID, prefixed `adiss_`.
    #[serde(default)]
    pub id: String,
    /// What the account running the campaign reads for the issue.
    #[serde(default)]
    pub message: String,
    /// The policy a `content_rejected` ad broke, or `null` when none was reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<AdIssuePolicy>,
    /// Why the ad or ad group can't deliver yet, as reported.
    pub reason: AdIssueReason,
    /// The ID of the ad or ad group the issue is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// The type of object the issue is on.
    pub resource_type: AdIssueResourceType,
    /// When the issue was last reported or changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl AdIssue {
    pub fn builder() -> AdIssueBuilder {
        <AdIssueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdIssueBuilder {
    created_at: Option<String>,
    id: Option<String>,
    message: Option<String>,
    policy: Option<AdIssuePolicy>,
    reason: Option<AdIssueReason>,
    resource_id: Option<String>,
    resource_type: Option<AdIssueResourceType>,
    updated_at: Option<String>,
}

impl AdIssueBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn policy(mut self, value: AdIssuePolicy) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn reason(mut self, value: AdIssueReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn resource_type(mut self, value: AdIssueResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdIssue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](AdIssueBuilder::created_at)
    /// - [`id`](AdIssueBuilder::id)
    /// - [`message`](AdIssueBuilder::message)
    /// - [`reason`](AdIssueBuilder::reason)
    /// - [`resource_type`](AdIssueBuilder::resource_type)
    /// - [`updated_at`](AdIssueBuilder::updated_at)
    pub fn build(self) -> Result<AdIssue, BuildError> {
        Ok(AdIssue {
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            policy: self.policy,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            resource_id: self.resource_id,
            resource_type: self
                .resource_type
                .ok_or_else(|| BuildError::missing_field("resource_type"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
