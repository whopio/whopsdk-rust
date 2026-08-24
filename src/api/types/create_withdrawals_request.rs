pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateWithdrawalsRequest {
    /// Set to true to continue when the bank could not confirm the account holder's name, or false to be refused in that case so the creator can fix the account or link their bank first. Omitting the argument skips the warning gate — a client that cannot show the warning keeps its pre-gate behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledge_bank_warning: Option<bool>,
    /// The amount to withdraw in the specified currency
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The ID of the company to withdraw from.
    #[serde(default)]
    pub company_id: String,
    /// The currency that is being withdrawn.
    pub currency: Currencies,
    /// A client-generated key that makes retries safe. Retrying with the same key returns the original withdrawal instead of creating a second one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// The ID of the payout method to use for the withdrawal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_method_id: Option<String>,
    /// Whether the platform covers the payout fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_covers_fees: Option<bool>,
    /// The processing speed for the withdrawal. Either standard or instant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<WithdrawalSpeeds>,
    /// Custom statement descriptor for the withdrawal. Must be between 5 and 22 characters and contain only alphanumeric characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

impl CreateWithdrawalsRequest {
    pub fn builder() -> CreateWithdrawalsRequestBuilder {
        <CreateWithdrawalsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWithdrawalsRequestBuilder {
    acknowledge_bank_warning: Option<bool>,
    amount: Option<f64>,
    company_id: Option<String>,
    currency: Option<Currencies>,
    idempotency_key: Option<String>,
    payout_method_id: Option<String>,
    platform_covers_fees: Option<bool>,
    speed: Option<WithdrawalSpeeds>,
    statement_descriptor: Option<String>,
}

impl CreateWithdrawalsRequestBuilder {
    pub fn acknowledge_bank_warning(mut self, value: bool) -> Self {
        self.acknowledge_bank_warning = Some(value);
        self
    }

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

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn payout_method_id(mut self, value: impl Into<String>) -> Self {
        self.payout_method_id = Some(value.into());
        self
    }

    pub fn platform_covers_fees(mut self, value: bool) -> Self {
        self.platform_covers_fees = Some(value);
        self
    }

    pub fn speed(mut self, value: WithdrawalSpeeds) -> Self {
        self.speed = Some(value);
        self
    }

    pub fn statement_descriptor(mut self, value: impl Into<String>) -> Self {
        self.statement_descriptor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWithdrawalsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateWithdrawalsRequestBuilder::amount)
    /// - [`company_id`](CreateWithdrawalsRequestBuilder::company_id)
    /// - [`currency`](CreateWithdrawalsRequestBuilder::currency)
    pub fn build(self) -> Result<CreateWithdrawalsRequest, BuildError> {
        Ok(CreateWithdrawalsRequest {
            acknowledge_bank_warning: self.acknowledge_bank_warning,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            idempotency_key: self.idempotency_key,
            payout_method_id: self.payout_method_id,
            platform_covers_fees: self.platform_covers_fees,
            speed: self.speed,
            statement_descriptor: self.statement_descriptor,
        })
    }
}
