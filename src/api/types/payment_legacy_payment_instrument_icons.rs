pub use crate::prelude::*;

/// The standard icon set: square and card shapes, each in light and dark colorways.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentLegacyPaymentInstrumentIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentLegacyPaymentInstrumentIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentLegacyPaymentInstrumentIconsSquare,
}

impl PaymentLegacyPaymentInstrumentIcons {
    pub fn builder() -> PaymentLegacyPaymentInstrumentIconsBuilder {
        <PaymentLegacyPaymentInstrumentIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLegacyPaymentInstrumentIconsBuilder {
    card: Option<PaymentLegacyPaymentInstrumentIconsCard>,
    square: Option<PaymentLegacyPaymentInstrumentIconsSquare>,
}

impl PaymentLegacyPaymentInstrumentIconsBuilder {
    pub fn card(mut self, value: PaymentLegacyPaymentInstrumentIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentLegacyPaymentInstrumentIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentLegacyPaymentInstrumentIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentLegacyPaymentInstrumentIconsBuilder::card)
    /// - [`square`](PaymentLegacyPaymentInstrumentIconsBuilder::square)
    pub fn build(self) -> Result<PaymentLegacyPaymentInstrumentIcons, BuildError> {
        Ok(PaymentLegacyPaymentInstrumentIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
