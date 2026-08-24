pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTopupsRequest {
    /// The amount to add to the balance in the specified currency. For example, 50.00 for $50.00 USD.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The unique identifier of the company to add funds to, starting with 'biz_'.
    #[serde(default)]
    pub company_id: String,
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
    amount: Option<f64>,
    company_id: Option<String>,
    currency: Option<Currencies>,
    payment_method_id: Option<String>,
}

impl CreateTopupsRequestBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
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
    /// - [`amount`](CreateTopupsRequestBuilder::amount)
    /// - [`company_id`](CreateTopupsRequestBuilder::company_id)
    /// - [`currency`](CreateTopupsRequestBuilder::currency)
    /// - [`payment_method_id`](CreateTopupsRequestBuilder::payment_method_id)
    pub fn build(self) -> Result<CreateTopupsRequest, BuildError> {
        Ok(CreateTopupsRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            payment_method_id: self
                .payment_method_id
                .ok_or_else(|| BuildError::missing_field("payment_method_id"))?,
        })
    }
}
