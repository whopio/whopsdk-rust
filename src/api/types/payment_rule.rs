pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentRule {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// What this account's rule requests when every condition matches. One applicable account-rule action wins, in this order: `allow`, `block`, `review`, `enforce_3ds`. An `allow` overrides this account's other rules, never Whop's own fraud controls. A `review` requests authorization without capture for an eligible on-session card payment through Whop Payments. Automatic capture is scheduled for 24 hours after authorization; capture or void the payment before then to decide sooner. Capture may complete later or fail. Review is skipped for unsupported methods, off-session payments, and payments already configured for manual capture. An `enforce_3ds` is skipped when the account rule cannot apply a challenge. Other 3DS requirements still apply.
    pub action: PaymentRuleAction,
    /// The conditions a payment is matched against. Up to 10 conditions, and 8 KiB once serialized.
    #[serde(default)]
    pub conditions: PaymentRuleConditions,
    /// When the rule was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// When the rule was deleted, as an ISO 8601 timestamp. `null` unless `status` is `deleted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    /// Payment rule ID, prefixed `prule_`.
    #[serde(default)]
    pub id: String,
    /// Custom string-to-string values for your integration. Maximum 50 keys, 40 characters per key, 500 characters per value.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// A name for this rule. Up to 255 characters.
    #[serde(default)]
    pub name: String,
    /// Whether the rule is applied to payments. A `deleted` rule is kept so the payments it already decided still name it.
    pub status: PaymentRuleStatus,
    /// When the rule was last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl PaymentRule {
    pub fn builder() -> PaymentRuleBuilder {
        <PaymentRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRuleBuilder {
    account_id: Option<String>,
    action: Option<PaymentRuleAction>,
    conditions: Option<PaymentRuleConditions>,
    created_at: Option<String>,
    deleted_at: Option<String>,
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    status: Option<PaymentRuleStatus>,
    updated_at: Option<String>,
}

impl PaymentRuleBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn action(mut self, value: PaymentRuleAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn conditions(mut self, value: PaymentRuleConditions) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn deleted_at(mut self, value: impl Into<String>) -> Self {
        self.deleted_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: PaymentRuleStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentRule`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](PaymentRuleBuilder::account_id)
    /// - [`action`](PaymentRuleBuilder::action)
    /// - [`conditions`](PaymentRuleBuilder::conditions)
    /// - [`created_at`](PaymentRuleBuilder::created_at)
    /// - [`id`](PaymentRuleBuilder::id)
    /// - [`metadata`](PaymentRuleBuilder::metadata)
    /// - [`name`](PaymentRuleBuilder::name)
    /// - [`status`](PaymentRuleBuilder::status)
    /// - [`updated_at`](PaymentRuleBuilder::updated_at)
    pub fn build(self) -> Result<PaymentRule, BuildError> {
        Ok(PaymentRule {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            conditions: self
                .conditions
                .ok_or_else(|| BuildError::missing_field("conditions"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
