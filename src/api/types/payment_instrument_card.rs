pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentInstrumentCard {
    /// The network identifier (`visa`, `amex`, …), matching `card.networks` entries and saved card payment methods.
    #[serde(default)]
    pub brand: String,
    /// The card's last four digits, when captured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}

impl PaymentInstrumentCard {
    pub fn builder() -> PaymentInstrumentCardBuilder {
        <PaymentInstrumentCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentInstrumentCardBuilder {
    brand: Option<String>,
    last4: Option<String>,
}

impl PaymentInstrumentCardBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentInstrumentCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`brand`](PaymentInstrumentCardBuilder::brand)
    pub fn build(self) -> Result<PaymentInstrumentCard, BuildError> {
        Ok(PaymentInstrumentCard {
            brand: self
                .brand
                .ok_or_else(|| BuildError::missing_field("brand"))?,
            last4: self.last4,
        })
    }
}
