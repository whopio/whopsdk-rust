pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodCashappPaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodCashappPaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodCashappPaymentMethodIconsSquare,
}

impl PaymentMethodCashappPaymentMethodIcons {
    pub fn builder() -> PaymentMethodCashappPaymentMethodIconsBuilder {
        <PaymentMethodCashappPaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodCashappPaymentMethodIconsBuilder {
    card: Option<PaymentMethodCashappPaymentMethodIconsCard>,
    square: Option<PaymentMethodCashappPaymentMethodIconsSquare>,
}

impl PaymentMethodCashappPaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodCashappPaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodCashappPaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodCashappPaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodCashappPaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodCashappPaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodCashappPaymentMethodIcons, BuildError> {
        Ok(PaymentMethodCashappPaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
