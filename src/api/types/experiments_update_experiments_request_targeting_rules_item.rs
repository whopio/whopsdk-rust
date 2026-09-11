pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateExperimentsRequestTargetingRulesItem {
    /// Conditions within this rule. Must be non-empty. All conditions must be satisfied for the rule to match (AND logic). Rules are OR-ed together.
    #[serde(default)]
    pub conditions: Vec<UpdateExperimentsRequestTargetingRulesItemConditionsItem>,
    /// `include` — users matching this rule qualify; `exclude` — users matching this rule are always excluded, overriding any include rule. Defaults to `include`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UpdateExperimentsRequestTargetingRulesItemType>,
}

impl UpdateExperimentsRequestTargetingRulesItem {
    pub fn builder() -> UpdateExperimentsRequestTargetingRulesItemBuilder {
        <UpdateExperimentsRequestTargetingRulesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExperimentsRequestTargetingRulesItemBuilder {
    conditions: Option<Vec<UpdateExperimentsRequestTargetingRulesItemConditionsItem>>,
    r#type: Option<UpdateExperimentsRequestTargetingRulesItemType>,
}

impl UpdateExperimentsRequestTargetingRulesItemBuilder {
    pub fn conditions(
        mut self,
        value: Vec<UpdateExperimentsRequestTargetingRulesItemConditionsItem>,
    ) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn r#type(mut self, value: UpdateExperimentsRequestTargetingRulesItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateExperimentsRequestTargetingRulesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conditions`](UpdateExperimentsRequestTargetingRulesItemBuilder::conditions)
    pub fn build(self) -> Result<UpdateExperimentsRequestTargetingRulesItem, BuildError> {
        Ok(UpdateExperimentsRequestTargetingRulesItem {
            conditions: self
                .conditions
                .ok_or_else(|| BuildError::missing_field("conditions"))?,
            r#type: self.r#type,
        })
    }
}
