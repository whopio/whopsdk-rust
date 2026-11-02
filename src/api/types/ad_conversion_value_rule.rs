pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdConversionValueRule {
    /// Business that owns this shared, editable rule.
    #[serde(default)]
    pub account_id: String,
    /// Set a fixed amount or change the original value by a signed percentage.
    pub adjustment_type: AdConversionValueRuleAdjustmentType,
    /// When the rule was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub events: Vec<AdConversionValueRuleEvent>,
    /// Amount sent for a fixed rule. Null for a percentage rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_value: Option<Money>,
    /// Conversion value rule ID, prefixed adcvr_.
    #[serde(default)]
    pub id: String,
    /// Free-form string keys and values for the caller.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// Signed percent change: 20 increases by 20%, negative 20 decreases by 20%. The sent value cannot go below zero. Null for a fixed rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percentage_change: Option<f64>,
    /// Whether this rule is active or paused.
    pub status: AdConversionValueRuleStatus,
    #[serde(default)]
    pub targets: Vec<AdConversionValueRuleTarget>,
    /// When the rule was edited, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl AdConversionValueRule {
    pub fn builder() -> AdConversionValueRuleBuilder {
        <AdConversionValueRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdConversionValueRuleBuilder {
    account_id: Option<String>,
    adjustment_type: Option<AdConversionValueRuleAdjustmentType>,
    created_at: Option<String>,
    events: Option<Vec<AdConversionValueRuleEvent>>,
    fixed_value: Option<Money>,
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    percentage_change: Option<f64>,
    status: Option<AdConversionValueRuleStatus>,
    targets: Option<Vec<AdConversionValueRuleTarget>>,
    updated_at: Option<String>,
}

impl AdConversionValueRuleBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn adjustment_type(mut self, value: AdConversionValueRuleAdjustmentType) -> Self {
        self.adjustment_type = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<AdConversionValueRuleEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn fixed_value(mut self, value: Money) -> Self {
        self.fixed_value = Some(value);
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

    pub fn percentage_change(mut self, value: f64) -> Self {
        self.percentage_change = Some(value);
        self
    }

    pub fn status(mut self, value: AdConversionValueRuleStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn targets(mut self, value: Vec<AdConversionValueRuleTarget>) -> Self {
        self.targets = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdConversionValueRule`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AdConversionValueRuleBuilder::account_id)
    /// - [`adjustment_type`](AdConversionValueRuleBuilder::adjustment_type)
    /// - [`created_at`](AdConversionValueRuleBuilder::created_at)
    /// - [`events`](AdConversionValueRuleBuilder::events)
    /// - [`id`](AdConversionValueRuleBuilder::id)
    /// - [`metadata`](AdConversionValueRuleBuilder::metadata)
    /// - [`status`](AdConversionValueRuleBuilder::status)
    /// - [`targets`](AdConversionValueRuleBuilder::targets)
    /// - [`updated_at`](AdConversionValueRuleBuilder::updated_at)
    pub fn build(self) -> Result<AdConversionValueRule, BuildError> {
        Ok(AdConversionValueRule {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            adjustment_type: self
                .adjustment_type
                .ok_or_else(|| BuildError::missing_field("adjustment_type"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            fixed_value: self.fixed_value,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            percentage_change: self.percentage_change,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            targets: self
                .targets
                .ok_or_else(|| BuildError::missing_field("targets"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
