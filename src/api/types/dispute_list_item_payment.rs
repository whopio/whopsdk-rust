pub use crate::prelude::*;

/// The original payment that was disputed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeListItemPayment {
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
    /// The instrument this payment was made with, shaped for display: the method type, a buyer-facing name, the standard icon set, and the card facts when it was a card. Null when the receipt names no payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_instrument: Option<DisputeListItemPaymentPaymentInstrument>,
}

impl DisputeListItemPayment {
    pub fn builder() -> DisputeListItemPaymentBuilder {
        <DisputeListItemPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeListItemPaymentBuilder {
    id: Option<String>,
    payment_instrument: Option<DisputeListItemPaymentPaymentInstrument>,
}

impl DisputeListItemPaymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn payment_instrument(mut self, value: DisputeListItemPaymentPaymentInstrument) -> Self {
        self.payment_instrument = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeListItemPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeListItemPaymentBuilder::id)
    pub fn build(self) -> Result<DisputeListItemPayment, BuildError> {
        Ok(DisputeListItemPayment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payment_instrument: self.payment_instrument,
        })
    }
}
