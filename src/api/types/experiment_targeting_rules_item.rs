pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExperimentTargetingRulesItem {
    /// Conditions within this rule, all of which must match.
    #[serde(default)]
    pub conditions: Vec<ExperimentTargetingRulesItemConditionsItem>,
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
    r#type: Option<ExperimentTargetingRulesItemType>,
}

impl ExperimentTargetingRulesItemBuilder {
    pub fn conditions(mut self, value: Vec<ExperimentTargetingRulesItemConditionsItem>) -> Self {
        self.conditions = Some(value);
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
            r#type: self.r#type,
        })
    }
}
