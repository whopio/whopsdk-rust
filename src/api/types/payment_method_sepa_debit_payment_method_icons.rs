pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodSepaDebitPaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodSepaDebitPaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodSepaDebitPaymentMethodIconsSquare,
}

impl PaymentMethodSepaDebitPaymentMethodIcons {
    pub fn builder() -> PaymentMethodSepaDebitPaymentMethodIconsBuilder {
        <PaymentMethodSepaDebitPaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodSepaDebitPaymentMethodIconsBuilder {
    card: Option<PaymentMethodSepaDebitPaymentMethodIconsCard>,
    square: Option<PaymentMethodSepaDebitPaymentMethodIconsSquare>,
}

impl PaymentMethodSepaDebitPaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodSepaDebitPaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodSepaDebitPaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodSepaDebitPaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodSepaDebitPaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodSepaDebitPaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodSepaDebitPaymentMethodIcons, BuildError> {
        Ok(PaymentMethodSepaDebitPaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
