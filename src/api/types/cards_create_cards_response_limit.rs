pub use crate::prelude::*;

/// The spending limit configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCardsResponseLimit {
    /// The limit amount in dollars.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The window the limit amount applies to. `per_transaction` caps each individual authorization and is what a limit set with `transaction_limit` reports.
    pub frequency: CreateCardsResponseLimitFrequency,
}

impl CreateCardsResponseLimit {
    pub fn builder() -> CreateCardsResponseLimitBuilder {
        <CreateCardsResponseLimitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCardsResponseLimitBuilder {
    amount: Option<f64>,
    frequency: Option<CreateCardsResponseLimitFrequency>,
}

impl CreateCardsResponseLimitBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn frequency(mut self, value: CreateCardsResponseLimitFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCardsResponseLimit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateCardsResponseLimitBuilder::amount)
    /// - [`frequency`](CreateCardsResponseLimitBuilder::frequency)
    pub fn build(self) -> Result<CreateCardsResponseLimit, BuildError> {
        Ok(CreateCardsResponseLimit {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
        })
    }
}
