pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinancingProviderCurrency {
    /// A currency checkout charges this provider in, lowercase ISO 4217.
    pub currency: FinancingProviderCurrencyCurrency,
    /// The largest payment checkout offers this provider for in this currency. Null when checkout sets no maximum, or when the provider sets each merchant's limit itself (Splitit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_amount: Option<Money>,
}

impl FinancingProviderCurrency {
    pub fn builder() -> FinancingProviderCurrencyBuilder {
        <FinancingProviderCurrencyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancingProviderCurrencyBuilder {
    currency: Option<FinancingProviderCurrencyCurrency>,
    max_amount: Option<Money>,
}

impl FinancingProviderCurrencyBuilder {
    pub fn currency(mut self, value: FinancingProviderCurrencyCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn max_amount(mut self, value: Money) -> Self {
        self.max_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinancingProviderCurrency`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](FinancingProviderCurrencyBuilder::currency)
    pub fn build(self) -> Result<FinancingProviderCurrency, BuildError> {
        Ok(FinancingProviderCurrency {
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            max_amount: self.max_amount,
        })
    }
}
