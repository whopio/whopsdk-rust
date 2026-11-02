pub use crate::prelude::*;

/// Standard-delivery estimate. Null if the method does not support standard delivery, or the amount does not cover the fee.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMethodsResponseDataItemQuoteStandard {
    /// Total fee charged, in the payout currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fee: f64,
    /// Amount remaining after fees, in the payout currency.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_received: f64,
}

impl ListMethodsResponseDataItemQuoteStandard {
    pub fn builder() -> ListMethodsResponseDataItemQuoteStandardBuilder {
        <ListMethodsResponseDataItemQuoteStandardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMethodsResponseDataItemQuoteStandardBuilder {
    fee: Option<f64>,
    total_received: Option<f64>,
}

impl ListMethodsResponseDataItemQuoteStandardBuilder {
    pub fn fee(mut self, value: f64) -> Self {
        self.fee = Some(value);
        self
    }

    pub fn total_received(mut self, value: f64) -> Self {
        self.total_received = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMethodsResponseDataItemQuoteStandard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fee`](ListMethodsResponseDataItemQuoteStandardBuilder::fee)
    /// - [`total_received`](ListMethodsResponseDataItemQuoteStandardBuilder::total_received)
    pub fn build(self) -> Result<ListMethodsResponseDataItemQuoteStandard, BuildError> {
        Ok(ListMethodsResponseDataItemQuoteStandard {
            fee: self.fee.ok_or_else(|| BuildError::missing_field("fee"))?,
            total_received: self
                .total_received
                .ok_or_else(|| BuildError::missing_field("total_received"))?,
        })
    }
}
