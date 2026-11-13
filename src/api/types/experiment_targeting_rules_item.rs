pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExperimentTargetingRulesItem {
    /// Conditions within this rule, all of which must match.
    #[serde(default)]
    pub conditions: Vec<ExperimentTargetingRulesItemConditionsItem>,
    /// Percentage of matching subjects that this rule lets in, from 0 to 100. Absent when the rule lets in every matching subject. Raising it keeps every subject that the rule already lets in.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pass_percentage: Option<f64>,
    /// `include` — users matching this rule qualify; `exclude` — users matching this rule are always excluded, overriding any include rule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ExperimentTargetingRulesItemType>,
}

impl ExperimentTargetingRulesItem {
    pub fn builder() -> ExperimentTargetingRulesItemBuilder {
        <ExperimentTargetingRulesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentTargetingRulesItemBuilder {
    conditions: Option<Vec<ExperimentTargetingRulesItemConditionsItem>>,
    pass_percentage: Option<f64>,
    r#type: Option<ExperimentTargetingRulesItemType>,
}

impl ExperimentTargetingRulesItemBuilder {
    pub fn conditions(mut self, value: Vec<ExperimentTargetingRulesItemConditionsItem>) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn pass_percentage(mut self, value: f64) -> Self {
        self.pass_percentage = Some(value);
        self
    }

    pub fn r#type(mut self, value: ExperimentTargetingRulesItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExperimentTargetingRulesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conditions`](ExperimentTargetingRulesItemBuilder::conditions)
    pub fn build(self) -> Result<ExperimentTargetingRulesItem, BuildError> {
        Ok(ExperimentTargetingRulesItem {
            conditions: self
                .conditions
                .ok_or_else(|| BuildError::missing_field("conditions"))?,
            pass_percentage: self.pass_percentage,
            r#type: self.r#type,
        })
    }
}
