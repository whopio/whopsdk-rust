pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemCardPaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodListItemCardPaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodListItemCardPaymentMethodIconsSquare,
}

impl PaymentMethodListItemCardPaymentMethodIcons {
    pub fn builder() -> PaymentMethodListItemCardPaymentMethodIconsBuilder {
        <PaymentMethodListItemCardPaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemCardPaymentMethodIconsBuilder {
    card: Option<PaymentMethodListItemCardPaymentMethodIconsCard>,
    square: Option<PaymentMethodListItemCardPaymentMethodIconsSquare>,
}

impl PaymentMethodListItemCardPaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodListItemCardPaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodListItemCardPaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemCardPaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodListItemCardPaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodListItemCardPaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodListItemCardPaymentMethodIcons, BuildError> {
        Ok(PaymentMethodListItemCardPaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
