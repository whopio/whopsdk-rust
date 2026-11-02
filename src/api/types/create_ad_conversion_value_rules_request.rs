pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAdConversionValueRulesRequest {
    /// Business that owns the rule.
    #[serde(default)]
    pub account_id: String,
    pub adjustment_type: CreateAdConversionValueRulesRequestAdjustmentType,
    /// Events adjusted on every selected target. Every platform must support every selected event.
    #[serde(default)]
    pub events: Vec<CreateAdConversionValueRulesRequestEventsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_value: Option<CreateAdConversionValueRulesRequestFixedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Signed percent change from negative 100 to 10000. The sent value cannot go below zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_change: Option<f64>,
    /// Exact IDs of active rules whose overlapping selections will be replaced. Other selections keep their values; remaining selections may split into separate rules. Broader rules remain as fallbacks for other items. Stale or incomplete conflict selections fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_rule_ids: Option<Vec<String>>,
    /// Initial rule status. Defaults to active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateAdConversionValueRulesRequestStatus>,
    /// Targets sharing one scope. Every selected event applies to every target. At most 500 target and event combinations.
    #[serde(default)]
    pub targets: Vec<CreateAdConversionValueRulesRequestTargetsItem>,
}

impl CreateAdConversionValueRulesRequest {
    pub fn builder() -> CreateAdConversionValueRulesRequestBuilder {
        <CreateAdConversionValueRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdConversionValueRulesRequestBuilder {
    account_id: Option<String>,
    adjustment_type: Option<CreateAdConversionValueRulesRequestAdjustmentType>,
    events: Option<Vec<CreateAdConversionValueRulesRequestEventsItem>>,
    fixed_value: Option<CreateAdConversionValueRulesRequestFixedValue>,
    metadata: Option<HashMap<String, String>>,
    percentage_change: Option<f64>,
    replace_rule_ids: Option<Vec<String>>,
    status: Option<CreateAdConversionValueRulesRequestStatus>,
    targets: Option<Vec<CreateAdConversionValueRulesRequestTargetsItem>>,
}

impl CreateAdConversionValueRulesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn adjustment_type(
        mut self,
        value: CreateAdConversionValueRulesRequestAdjustmentType,
    ) -> Self {
        self.adjustment_type = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<CreateAdConversionValueRulesRequestEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn fixed_value(mut self, value: CreateAdConversionValueRulesRequestFixedValue) -> Self {
        self.fixed_value = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn percentage_change(mut self, value: f64) -> Self {
        self.percentage_change = Some(value);
        self
    }

    pub fn replace_rule_ids(mut self, value: Vec<String>) -> Self {
        self.replace_rule_ids = Some(value);
        self
    }

    pub fn status(mut self, value: CreateAdConversionValueRulesRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn targets(mut self, value: Vec<CreateAdConversionValueRulesRequestTargetsItem>) -> Self {
        self.targets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAdConversionValueRulesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateAdConversionValueRulesRequestBuilder::account_id)
    /// - [`adjustment_type`](CreateAdConversionValueRulesRequestBuilder::adjustment_type)
    /// - [`events`](CreateAdConversionValueRulesRequestBuilder::events)
    /// - [`targets`](CreateAdConversionValueRulesRequestBuilder::targets)
    pub fn build(self) -> Result<CreateAdConversionValueRulesRequest, BuildError> {
        Ok(CreateAdConversionValueRulesRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            adjustment_type: self
                .adjustment_type
                .ok_or_else(|| BuildError::missing_field("adjustment_type"))?,
            events: self
                .events
                .ok_or_else(|| BuildError::missing_field("events"))?,
            fixed_value: self.fixed_value,
            metadata: self.metadata,
            percentage_change: self.percentage_change,
            replace_rule_ids: self.replace_rule_ids,
            status: self.status,
            targets: self
                .targets
                .ok_or_else(|| BuildError::missing_field("targets"))?,
        })
    }
}
