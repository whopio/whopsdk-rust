pub use crate::prelude::*;

/// Instant-delivery estimate. Null if the method does not support instant delivery, instant delivery is unavailable for the account, or the amount does not cover the fee.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PostPayoutMethodCreatedPayloadDataQuoteInstant {
    /// Total fee charged, in the withdrawal currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fee: f64,
    /// Amount delivered after fees, in the withdrawal currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_received: f64,
}

impl PostPayoutMethodCreatedPayloadDataQuoteInstant {
    pub fn builder() -> PostPayoutMethodCreatedPayloadDataQuoteInstantBuilder {
        <PostPayoutMethodCreatedPayloadDataQuoteInstantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostPayoutMethodCreatedPayloadDataQuoteInstantBuilder {
    fee: Option<f64>,
    total_received: Option<f64>,
}

impl PostPayoutMethodCreatedPayloadDataQuoteInstantBuilder {
    pub fn fee(mut self, value: f64) -> Self {
        self.fee = Some(value);
        self
    }

    pub fn total_received(mut self, value: f64) -> Self {
        self.total_received = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostPayoutMethodCreatedPayloadDataQuoteInstant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fee`](PostPayoutMethodCreatedPayloadDataQuoteInstantBuilder::fee)
    /// - [`total_received`](PostPayoutMethodCreatedPayloadDataQuoteInstantBuilder::total_received)
    pub fn build(self) -> Result<PostPayoutMethodCreatedPayloadDataQuoteInstant, BuildError> {
        Ok(PostPayoutMethodCreatedPayloadDataQuoteInstant {
            fee: self.fee.ok_or_else(|| BuildError::missing_field("fee"))?,
            total_received: self
                .total_received
                .ok_or_else(|| BuildError::missing_field("total_received"))?,
        })
    }
}
