pub use crate::prelude::*;

/// The standard icon set: square and card shapes, each in light and dark colorways.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentPaymentInstrumentIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentPaymentInstrumentIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentPaymentInstrumentIconsSquare,
}

impl PaymentPaymentInstrumentIcons {
    pub fn builder() -> PaymentPaymentInstrumentIconsBuilder {
        <PaymentPaymentInstrumentIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPaymentInstrumentIconsBuilder {
    card: Option<PaymentPaymentInstrumentIconsCard>,
    square: Option<PaymentPaymentInstrumentIconsSquare>,
}

impl PaymentPaymentInstrumentIconsBuilder {
    pub fn card(mut self, value: PaymentPaymentInstrumentIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentPaymentInstrumentIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentPaymentInstrumentIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentPaymentInstrumentIconsBuilder::card)
    /// - [`square`](PaymentPaymentInstrumentIconsBuilder::square)
    pub fn build(self) -> Result<PaymentPaymentInstrumentIcons, BuildError> {
        Ok(PaymentPaymentInstrumentIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
