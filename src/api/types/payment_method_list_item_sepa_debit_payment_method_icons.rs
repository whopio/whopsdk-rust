pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemSepaDebitPaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodListItemSepaDebitPaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodListItemSepaDebitPaymentMethodIconsSquare,
}

impl PaymentMethodListItemSepaDebitPaymentMethodIcons {
    pub fn builder() -> PaymentMethodListItemSepaDebitPaymentMethodIconsBuilder {
        <PaymentMethodListItemSepaDebitPaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemSepaDebitPaymentMethodIconsBuilder {
    card: Option<PaymentMethodListItemSepaDebitPaymentMethodIconsCard>,
    square: Option<PaymentMethodListItemSepaDebitPaymentMethodIconsSquare>,
}

impl PaymentMethodListItemSepaDebitPaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodListItemSepaDebitPaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodListItemSepaDebitPaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemSepaDebitPaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodListItemSepaDebitPaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodListItemSepaDebitPaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodListItemSepaDebitPaymentMethodIcons, BuildError> {
        Ok(PaymentMethodListItemSepaDebitPaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
