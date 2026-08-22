pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemCashappPaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodListItemCashappPaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodListItemCashappPaymentMethodIconsSquare,
}

impl PaymentMethodListItemCashappPaymentMethodIcons {
    pub fn builder() -> PaymentMethodListItemCashappPaymentMethodIconsBuilder {
        <PaymentMethodListItemCashappPaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemCashappPaymentMethodIconsBuilder {
    card: Option<PaymentMethodListItemCashappPaymentMethodIconsCard>,
    square: Option<PaymentMethodListItemCashappPaymentMethodIconsSquare>,
}

impl PaymentMethodListItemCashappPaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodListItemCashappPaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodListItemCashappPaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemCashappPaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodListItemCashappPaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodListItemCashappPaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodListItemCashappPaymentMethodIcons, BuildError> {
        Ok(PaymentMethodListItemCashappPaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
