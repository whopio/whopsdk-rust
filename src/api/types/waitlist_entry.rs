pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WaitlistEntry {
    /// The seller account, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Why the last approval attempt failed, or `null` when none has. `plan_unavailable` — the plan, product, or seller account was deleted. `already_member` — the user already has a membership on a one-per-user product. `checkout_failed` — checkout failed, usually a declined payment, and the signup was denied. `unknown` — another failure; retry. Cleared when approval is requeued.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_failure_reason: Option<WaitlistEntryApprovalFailureReason>,
    /// The account the signup was submitted on behalf of, prefixed `biz_`, or `null` when the user signed up for themselves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_account_id: Option<String>,
    /// When this signup was submitted, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub custom_field_responses: Vec<WaitlistEntryCustomFieldResponse>,
    /// The waitlist signup ID, prefixed `entry_`.
    #[serde(default)]
    pub id: String,
    /// The membership created for this signup, prefixed `mem_`, or `null` when there is none. Check the membership's status to determine access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    /// Custom key-value metadata associated with this signup.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// The plan this signup belongs to, prefixed `plan_`.
    #[serde(default)]
    pub plan_id: String,
    /// The product this signup belongs to, prefixed `prod_`, or `null` when the plan has no product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The signup's current state. Approval runs asynchronously, so a signup stays `pending` until processing completes. `approved` alone does not prove an active membership; check `membership_id`.
    pub status: WaitlistEntryStatus,
    /// When this signup last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The user who submitted this signup, prefixed `user_`.
    #[serde(default)]
    pub user_id: String,
}

impl WaitlistEntry {
    pub fn builder() -> WaitlistEntryBuilder {
        <WaitlistEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaitlistEntryBuilder {
    account_id: Option<String>,
    approval_failure_reason: Option<WaitlistEntryApprovalFailureReason>,
    buyer_account_id: Option<String>,
    created_at: Option<String>,
    custom_field_responses: Option<Vec<WaitlistEntryCustomFieldResponse>>,
    id: Option<String>,
    membership_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    plan_id: Option<String>,
    product_id: Option<String>,
    status: Option<WaitlistEntryStatus>,
    updated_at: Option<String>,
    user_id: Option<String>,
}

impl WaitlistEntryBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn approval_failure_reason(mut self, value: WaitlistEntryApprovalFailureReason) -> Self {
        self.approval_failure_reason = Some(value);
        self
    }

    pub fn buyer_account_id(mut self, value: impl Into<String>) -> Self {
        self.buyer_account_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn custom_field_responses(mut self, value: Vec<WaitlistEntryCustomFieldResponse>) -> Self {
        self.custom_field_responses = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn membership_id(mut self, value: impl Into<String>) -> Self {
        self.membership_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: WaitlistEntryStatus) -> Self {
        self.status = Some(value);
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

    /// Consumes the builder and constructs a [`WaitlistEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](WaitlistEntryBuilder::account_id)
    /// - [`created_at`](WaitlistEntryBuilder::created_at)
    /// - [`custom_field_responses`](WaitlistEntryBuilder::custom_field_responses)
    /// - [`id`](WaitlistEntryBuilder::id)
    /// - [`metadata`](WaitlistEntryBuilder::metadata)
    /// - [`plan_id`](WaitlistEntryBuilder::plan_id)
    /// - [`status`](WaitlistEntryBuilder::status)
    /// - [`updated_at`](WaitlistEntryBuilder::updated_at)
    /// - [`user_id`](WaitlistEntryBuilder::user_id)
    pub fn build(self) -> Result<WaitlistEntry, BuildError> {
        Ok(WaitlistEntry {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            approval_failure_reason: self.approval_failure_reason,
            buyer_account_id: self.buyer_account_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            custom_field_responses: self
                .custom_field_responses
                .ok_or_else(|| BuildError::missing_field("custom_field_responses"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            membership_id: self.membership_id,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            plan_id: self
                .plan_id
                .ok_or_else(|| BuildError::missing_field("plan_id"))?,
            product_id: self.product_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
