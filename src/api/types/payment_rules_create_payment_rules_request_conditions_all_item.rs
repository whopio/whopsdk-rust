pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePaymentRulesRequestConditionsAllItem {
    /// The payment attribute this condition reads.
    pub field: CreatePaymentRulesRequestConditionsAllItemField,
    /// How the payment attribute is compared to the value.
    pub operator: CreatePaymentRulesRequestConditionsAllItemOperator,
    pub value: PaymentRuleConditionValue,
}

impl CreatePaymentRulesRequestConditionsAllItem {
    pub fn builder() -> CreatePaymentRulesRequestConditionsAllItemBuilder {
        <CreatePaymentRulesRequestConditionsAllItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentRulesRequestConditionsAllItemBuilder {
    field: Option<CreatePaymentRulesRequestConditionsAllItemField>,
    operator: Option<CreatePaymentRulesRequestConditionsAllItemOperator>,
    value: Option<PaymentRuleConditionValue>,
}

impl CreatePaymentRulesRequestConditionsAllItemBuilder {
    pub fn field(mut self, value: CreatePaymentRulesRequestConditionsAllItemField) -> Self {
        self.field = Some(value);
        self
    }

    pub fn operator(mut self, value: CreatePaymentRulesRequestConditionsAllItemOperator) -> Self {
        self.operator = Some(value);
        self
    }

    pub fn value(mut self, value: PaymentRuleConditionValue) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentRulesRequestConditionsAllItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](CreatePaymentRulesRequestConditionsAllItemBuilder::field)
    /// - [`operator`](CreatePaymentRulesRequestConditionsAllItemBuilder::operator)
    /// - [`value`](CreatePaymentRulesRequestConditionsAllItemBuilder::value)
    pub fn build(self) -> Result<CreatePaymentRulesRequestConditionsAllItem, BuildError> {
        Ok(CreatePaymentRulesRequestConditionsAllItem {
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
