pub use crate::prelude::*;

/// The standard icon set: square and card shapes, each in light and dark colorways.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentListItemPaymentInstrumentIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentListItemPaymentInstrumentIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentListItemPaymentInstrumentIconsSquare,
}

impl PaymentListItemPaymentInstrumentIcons {
    pub fn builder() -> PaymentListItemPaymentInstrumentIconsBuilder {
        <PaymentListItemPaymentInstrumentIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentListItemPaymentInstrumentIconsBuilder {
    card: Option<PaymentListItemPaymentInstrumentIconsCard>,
    square: Option<PaymentListItemPaymentInstrumentIconsSquare>,
}

impl PaymentListItemPaymentInstrumentIconsBuilder {
    pub fn card(mut self, value: PaymentListItemPaymentInstrumentIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentListItemPaymentInstrumentIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentListItemPaymentInstrumentIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentListItemPaymentInstrumentIconsBuilder::card)
    /// - [`square`](PaymentListItemPaymentInstrumentIconsBuilder::square)
    pub fn build(self) -> Result<PaymentListItemPaymentInstrumentIcons, BuildError> {
        Ok(PaymentListItemPaymentInstrumentIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
