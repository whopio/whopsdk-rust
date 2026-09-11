pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExperimentTargetingRulesItemConditionsItem {
    /// Property name to read from the user context. Present when `type` is `property`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Comparison to apply.
    pub operator: ExperimentTargetingRulesItemConditionsItemOperator,
    /// What the condition matches on: the user ID, the account ID, or a named user property.
    pub r#type: ExperimentTargetingRulesItemConditionsItemType,
    pub value: serde_json::Value,
}

impl ExperimentTargetingRulesItemConditionsItem {
    pub fn builder() -> ExperimentTargetingRulesItemConditionsItemBuilder {
        <ExperimentTargetingRulesItemConditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentTargetingRulesItemConditionsItemBuilder {
    field: Option<String>,
    operator: Option<ExperimentTargetingRulesItemConditionsItemOperator>,
    r#type: Option<ExperimentTargetingRulesItemConditionsItemType>,
    value: Option<serde_json::Value>,
}

impl ExperimentTargetingRulesItemConditionsItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn operator(mut self, value: ExperimentTargetingRulesItemConditionsItemOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn r#type(mut self, value: ExperimentTargetingRulesItemConditionsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExperimentTargetingRulesItemConditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operator`](ExperimentTargetingRulesItemConditionsItemBuilder::operator)
    /// - [`r#type`](ExperimentTargetingRulesItemConditionsItemBuilder::r#type)
    /// - [`value`](ExperimentTargetingRulesItemConditionsItemBuilder::value)
    pub fn build(self) -> Result<ExperimentTargetingRulesItemConditionsItem, BuildError> {
        Ok(ExperimentTargetingRulesItemConditionsItem {
            field: self.field,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
