pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentRuleCondition {
    /// The payment attribute this condition reads.
    pub field: PaymentRuleConditionField,
    /// How the payment attribute is compared to the value.
    pub operator: PaymentRuleConditionOperator,
    /// The value to compare against.
    pub value: PaymentRuleConditionValue,
}

impl PaymentRuleCondition {
    pub fn builder() -> PaymentRuleConditionBuilder {
        <PaymentRuleConditionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRuleConditionBuilder {
    field: Option<PaymentRuleConditionField>,
    operator: Option<PaymentRuleConditionOperator>,
    value: Option<PaymentRuleConditionValue>,
}

impl PaymentRuleConditionBuilder {
    pub fn field(mut self, value: PaymentRuleConditionField) -> Self {
        self.field = Some(value);
        self
    }

    pub fn operator(mut self, value: PaymentRuleConditionOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn value(mut self, value: PaymentRuleConditionValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentRuleCondition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PaymentRuleConditionBuilder::field)
    /// - [`operator`](PaymentRuleConditionBuilder::operator)
    /// - [`value`](PaymentRuleConditionBuilder::value)
    pub fn build(self) -> Result<PaymentRuleCondition, BuildError> {
        Ok(PaymentRuleCondition {
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
