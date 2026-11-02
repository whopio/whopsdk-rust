pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReplacePaymentRulesRequestConditionsAllItem {
    /// The payment attribute this condition reads.
    pub field: ReplacePaymentRulesRequestConditionsAllItemField,
    /// How the payment attribute is compared to the value.
    pub operator: ReplacePaymentRulesRequestConditionsAllItemOperator,
    pub value: PaymentRuleConditionValue,
}

impl ReplacePaymentRulesRequestConditionsAllItem {
    pub fn builder() -> ReplacePaymentRulesRequestConditionsAllItemBuilder {
        <ReplacePaymentRulesRequestConditionsAllItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplacePaymentRulesRequestConditionsAllItemBuilder {
    field: Option<ReplacePaymentRulesRequestConditionsAllItemField>,
    operator: Option<ReplacePaymentRulesRequestConditionsAllItemOperator>,
    value: Option<PaymentRuleConditionValue>,
}

impl ReplacePaymentRulesRequestConditionsAllItemBuilder {
    pub fn field(mut self, value: ReplacePaymentRulesRequestConditionsAllItemField) -> Self {
        self.field = Some(value);
        self
    }

    pub fn operator(mut self, value: ReplacePaymentRulesRequestConditionsAllItemOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn value(mut self, value: PaymentRuleConditionValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplacePaymentRulesRequestConditionsAllItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ReplacePaymentRulesRequestConditionsAllItemBuilder::field)
    /// - [`operator`](ReplacePaymentRulesRequestConditionsAllItemBuilder::operator)
    /// - [`value`](ReplacePaymentRulesRequestConditionsAllItemBuilder::value)
    pub fn build(self) -> Result<ReplacePaymentRulesRequestConditionsAllItem, BuildError> {
        Ok(ReplacePaymentRulesRequestConditionsAllItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            operator: self
                .operator
                .ok_or_else(|| BuildError::missing_field("operator"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
