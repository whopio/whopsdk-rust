pub use crate::prelude::*;

/// Price answer, such as `max_product_price`. `null` clears the price.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateFinancingApplicationsRequestAnswersItemMoney {
    /// Positive exact decimal amount in major units, such as 1234.56. At most 12 whole digits and the currency's fractional precision.
    #[serde(default)]
    pub amount: String,
    /// Currency of the maximum product price.
    pub currency: UpdateFinancingApplicationsRequestAnswersItemMoneyCurrency,
}

impl UpdateFinancingApplicationsRequestAnswersItemMoney {
    pub fn builder() -> UpdateFinancingApplicationsRequestAnswersItemMoneyBuilder {
        <UpdateFinancingApplicationsRequestAnswersItemMoneyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFinancingApplicationsRequestAnswersItemMoneyBuilder {
    amount: Option<String>,
    currency: Option<UpdateFinancingApplicationsRequestAnswersItemMoneyCurrency>,
}

impl UpdateFinancingApplicationsRequestAnswersItemMoneyBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn currency(
        mut self,
        value: UpdateFinancingApplicationsRequestAnswersItemMoneyCurrency,
    ) -> Self {
        self.currency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFinancingApplicationsRequestAnswersItemMoney`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](UpdateFinancingApplicationsRequestAnswersItemMoneyBuilder::amount)
    /// - [`currency`](UpdateFinancingApplicationsRequestAnswersItemMoneyBuilder::currency)
    pub fn build(self) -> Result<UpdateFinancingApplicationsRequestAnswersItemMoney, BuildError> {
        Ok(UpdateFinancingApplicationsRequestAnswersItemMoney {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
        })
    }
}
