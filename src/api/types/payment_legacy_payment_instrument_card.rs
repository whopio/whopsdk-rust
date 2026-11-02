pub use crate::prelude::*;

/// Card payments only: the card's network and last four.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentLegacyPaymentInstrumentCard {
    /// The network identifier (`visa`, `amex`, …), matching `card.networks` entries and saved card payment methods.
    #[serde(default)]
    pub brand: String,
    /// The card's last four digits, when captured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

impl PaymentLegacyPaymentInstrumentCard {
    pub fn builder() -> PaymentLegacyPaymentInstrumentCardBuilder {
        <PaymentLegacyPaymentInstrumentCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLegacyPaymentInstrumentCardBuilder {
    brand: Option<String>,
    last4: Option<String>,
}

impl PaymentLegacyPaymentInstrumentCardBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentLegacyPaymentInstrumentCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brand`](PaymentLegacyPaymentInstrumentCardBuilder::brand)
    pub fn build(self) -> Result<PaymentLegacyPaymentInstrumentCard, BuildError> {
        Ok(PaymentLegacyPaymentInstrumentCard {
            brand: self
                .brand
                .ok_or_else(|| BuildError::missing_field("brand"))?,
            last4: self.last4,
        })
    }
}
