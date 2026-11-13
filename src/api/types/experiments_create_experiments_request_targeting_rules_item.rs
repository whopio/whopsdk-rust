pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateExperimentsRequestTargetingRulesItem {
    /// Conditions within this rule. Must be non-empty. All conditions must be satisfied for the rule to match (AND logic). Rules are OR-ed together.
    #[serde(default)]
    pub conditions: Vec<CreateExperimentsRequestTargetingRulesItemConditionsItem>,
    /// Percentage of matching subjects that this rule lets in, from 0 to 100. Omit it to let in every matching subject. Only `include` rules accept it. Raising it keeps every subject that the rule already lets in.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub pass_percentage: Option<f64>,
    /// `include` — users matching this rule qualify; `exclude` — users matching this rule are always excluded, overriding any include rule. Defaults to `include`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CreateExperimentsRequestTargetingRulesItemType>,
}

impl CreateExperimentsRequestTargetingRulesItem {
    pub fn builder() -> CreateExperimentsRequestTargetingRulesItemBuilder {
        <CreateExperimentsRequestTargetingRulesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExperimentsRequestTargetingRulesItemBuilder {
    conditions: Option<Vec<CreateExperimentsRequestTargetingRulesItemConditionsItem>>,
    pass_percentage: Option<f64>,
    r#type: Option<CreateExperimentsRequestTargetingRulesItemType>,
}

impl CreateExperimentsRequestTargetingRulesItemBuilder {
    pub fn conditions(
        mut self,
        value: Vec<CreateExperimentsRequestTargetingRulesItemConditionsItem>,
    ) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn pass_percentage(mut self, value: f64) -> Self {
        self.pass_percentage = Some(value);
        self
    }

    pub fn r#type(mut self, value: CreateExperimentsRequestTargetingRulesItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateExperimentsRequestTargetingRulesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conditions`](CreateExperimentsRequestTargetingRulesItemBuilder::conditions)
    pub fn build(self) -> Result<CreateExperimentsRequestTargetingRulesItem, BuildError> {
        Ok(CreateExperimentsRequestTargetingRulesItem {
            conditions: self
                .conditions
                .ok_or_else(|| BuildError::missing_field("conditions"))?,
            pass_percentage: self.pass_percentage,
            r#type: self.r#type,
        })
    }
}
