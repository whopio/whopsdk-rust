pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAdConversionValueRulesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustment_type: Option<UpdateAdConversionValueRulesRequestAdjustmentType>,
    /// Events adjusted on every selected target. Every platform must support every selected event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<UpdateAdConversionValueRulesRequestEventsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_value: Option<UpdateAdConversionValueRulesRequestFixedValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Signed percent change from negative 100 to 10000. The sent value cannot go below zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_change: Option<f64>,
    /// Exact IDs of active rules whose overlapping selections will be replaced. Other selections keep their values; remaining selections may split into separate rules. Broader rules remain as fallbacks for other items. Stale or incomplete conflict selections fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_rule_ids: Option<Vec<String>>,
    /// Targets sharing one scope. Every selected event applies to every target. At most 500 target and event combinations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<UpdateAdConversionValueRulesRequestTargetsItem>>,
}

impl UpdateAdConversionValueRulesRequest {
    pub fn builder() -> UpdateAdConversionValueRulesRequestBuilder {
        <UpdateAdConversionValueRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdConversionValueRulesRequestBuilder {
    adjustment_type: Option<UpdateAdConversionValueRulesRequestAdjustmentType>,
    events: Option<Vec<UpdateAdConversionValueRulesRequestEventsItem>>,
    fixed_value: Option<UpdateAdConversionValueRulesRequestFixedValue>,
    metadata: Option<HashMap<String, String>>,
    percentage_change: Option<f64>,
    replace_rule_ids: Option<Vec<String>>,
    targets: Option<Vec<UpdateAdConversionValueRulesRequestTargetsItem>>,
}

impl UpdateAdConversionValueRulesRequestBuilder {
    pub fn adjustment_type(
        mut self,
        value: UpdateAdConversionValueRulesRequestAdjustmentType,
    ) -> Self {
        self.adjustment_type = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<UpdateAdConversionValueRulesRequestEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn fixed_value(mut self, value: UpdateAdConversionValueRulesRequestFixedValue) -> Self {
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

    pub fn targets(mut self, value: Vec<UpdateAdConversionValueRulesRequestTargetsItem>) -> Self {
        self.targets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdConversionValueRulesRequest`].
    pub fn build(self) -> Result<UpdateAdConversionValueRulesRequest, BuildError> {
        Ok(UpdateAdConversionValueRulesRequest {
            adjustment_type: self.adjustment_type,
            events: self.events,
            fixed_value: self.fixed_value,
            metadata: self.metadata,
            percentage_change: self.percentage_change,
            replace_rule_ids: self.replace_rule_ids,
            targets: self.targets,
        })
    }
}
