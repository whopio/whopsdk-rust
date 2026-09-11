pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateExperimentsRequestTargetingRulesItem {
    /// Conditions within this rule. Must be non-empty. All conditions must be satisfied for the rule to match (AND logic). Rules are OR-ed together.
    #[serde(default)]
    pub conditions: Vec<CreateExperimentsRequestTargetingRulesItemConditionsItem>,
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
            r#type: self.r#type,
        })
    }
}
