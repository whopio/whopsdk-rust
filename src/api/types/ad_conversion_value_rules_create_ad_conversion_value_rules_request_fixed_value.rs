pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdConversionValueRulesRequestFixedValue {
    /// Decimal amount with at most 2 decimal places. Fixed values must be greater than 0 and at most 99999999.99.
    #[serde(default)]
    pub amount: String,
    #[serde(default)]
    pub currency: String,
}

impl CreateAdConversionValueRulesRequestFixedValue {
    pub fn builder() -> CreateAdConversionValueRulesRequestFixedValueBuilder {
        <CreateAdConversionValueRulesRequestFixedValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdConversionValueRulesRequestFixedValueBuilder {
    amount: Option<String>,
    currency: Option<String>,
}

impl CreateAdConversionValueRulesRequestFixedValueBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdConversionValueRulesRequestFixedValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateAdConversionValueRulesRequestFixedValueBuilder::amount)
    /// - [`currency`](CreateAdConversionValueRulesRequestFixedValueBuilder::currency)
    pub fn build(self) -> Result<CreateAdConversionValueRulesRequestFixedValue, BuildError> {
        Ok(CreateAdConversionValueRulesRequestFixedValue {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
        })
    }
}
