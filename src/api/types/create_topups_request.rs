pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTopupsRequest {
    /// The unique identifier of the company to add funds to, starting with 'biz_'.
    #[serde(default)]
    pub account_id: String,
    /// The amount to add to the balance in the specified currency. For example, 50.00 for $50.00 USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The currency for the top-up amount, such as 'usd'.
    pub currency: Currencies,
    /// The unique identifier of the stored payment method to charge for the top-up.
    #[serde(default)]
    pub payment_method_id: String,
}

impl CreateTopupsRequest {
    pub fn builder() -> CreateTopupsRequestBuilder {
        <CreateTopupsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTopupsRequestBuilder {
    account_id: Option<String>,
    amount: Option<f64>,
    currency: Option<Currencies>,
    payment_method_id: Option<String>,
}

impl CreateTopupsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTopupsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateTopupsRequestBuilder::account_id)
    /// - [`amount`](CreateTopupsRequestBuilder::amount)
    /// - [`currency`](CreateTopupsRequestBuilder::currency)
    /// - [`payment_method_id`](CreateTopupsRequestBuilder::payment_method_id)
    pub fn build(self) -> Result<CreateTopupsRequest, BuildError> {
        Ok(CreateTopupsRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            payment_method_id: self
                .payment_method_id
                .ok_or_else(|| BuildError::missing_field("payment_method_id"))?,
        })
    }
}
