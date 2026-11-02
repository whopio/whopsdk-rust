pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateExperimentsRequestTargetingRulesItemConditionsItem {
    /// Property name to read from the user context. Required when `type` is `property`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Comparison to apply: `any`/`none` — inclusion/exclusion list; `eq`/`neq` — equality; `gt`/`gte`/`lt`/`lte` — numeric range.
    pub operator: UpdateExperimentsRequestTargetingRulesItemConditionsItemOperator,
    /// `user_id` — match on subject user ID; `account_id` — match on account ID (prefixed `biz_`); `property` — match on a named user property (requires `field`).
    pub r#type: UpdateExperimentsRequestTargetingRulesItemConditionsItemType,
    pub value: serde_json::Value,
}

impl UpdateExperimentsRequestTargetingRulesItemConditionsItem {
    pub fn builder() -> UpdateExperimentsRequestTargetingRulesItemConditionsItemBuilder {
        <UpdateExperimentsRequestTargetingRulesItemConditionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExperimentsRequestTargetingRulesItemConditionsItemBuilder {
    field: Option<String>,
    operator: Option<UpdateExperimentsRequestTargetingRulesItemConditionsItemOperator>,
    r#type: Option<UpdateExperimentsRequestTargetingRulesItemConditionsItemType>,
    value: Option<serde_json::Value>,
}

impl UpdateExperimentsRequestTargetingRulesItemConditionsItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn operator(
        mut self,
        value: UpdateExperimentsRequestTargetingRulesItemConditionsItemOperator,
    ) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn r#type(
        mut self,
        value: UpdateExperimentsRequestTargetingRulesItemConditionsItemType,
    ) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateExperimentsRequestTargetingRulesItemConditionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operator`](UpdateExperimentsRequestTargetingRulesItemConditionsItemBuilder::operator)
    /// - [`r#type`](UpdateExperimentsRequestTargetingRulesItemConditionsItemBuilder::r#type)
    /// - [`value`](UpdateExperimentsRequestTargetingRulesItemConditionsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<UpdateExperimentsRequestTargetingRulesItemConditionsItem, BuildError> {
        Ok(UpdateExperimentsRequestTargetingRulesItemConditionsItem {
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
