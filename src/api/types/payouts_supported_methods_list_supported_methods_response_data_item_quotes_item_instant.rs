pub use crate::prelude::*;

/// Instant-delivery estimate. Null if unsupported, unavailable for the account, or the amount does not cover the fee.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSupportedMethodsResponseDataItemQuotesItemInstant {
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub estimated_arrival: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub fee: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_received: f64,
}

impl ListSupportedMethodsResponseDataItemQuotesItemInstant {
    pub fn builder() -> ListSupportedMethodsResponseDataItemQuotesItemInstantBuilder {
        <ListSupportedMethodsResponseDataItemQuotesItemInstantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSupportedMethodsResponseDataItemQuotesItemInstantBuilder {
    estimated_arrival: Option<DateTime<FixedOffset>>,
    fee: Option<f64>,
    total_received: Option<f64>,
}

impl ListSupportedMethodsResponseDataItemQuotesItemInstantBuilder {
    pub fn estimated_arrival(mut self, value: DateTime<FixedOffset>) -> Self {
        self.estimated_arrival = Some(value);
        self
    }

    pub fn fee(mut self, value: f64) -> Self {
        self.fee = Some(value);
        self
    }

    pub fn total_received(mut self, value: f64) -> Self {
        self.total_received = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSupportedMethodsResponseDataItemQuotesItemInstant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`estimated_arrival`](ListSupportedMethodsResponseDataItemQuotesItemInstantBuilder::estimated_arrival)
    /// - [`fee`](ListSupportedMethodsResponseDataItemQuotesItemInstantBuilder::fee)
    /// - [`total_received`](ListSupportedMethodsResponseDataItemQuotesItemInstantBuilder::total_received)
    pub fn build(
        self,
    ) -> Result<ListSupportedMethodsResponseDataItemQuotesItemInstant, BuildError> {
        Ok(ListSupportedMethodsResponseDataItemQuotesItemInstant {
            estimated_arrival: self
                .estimated_arrival
                .ok_or_else(|| BuildError::missing_field("estimated_arrival"))?,
            fee: self.fee.ok_or_else(|| BuildError::missing_field("fee"))?,
            total_received: self
                .total_received
                .ok_or_else(|| BuildError::missing_field("total_received"))?,
        })
    }
}
