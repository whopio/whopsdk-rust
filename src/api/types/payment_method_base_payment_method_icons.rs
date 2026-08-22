pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodBasePaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodBasePaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodBasePaymentMethodIconsSquare,
}

impl PaymentMethodBasePaymentMethodIcons {
    pub fn builder() -> PaymentMethodBasePaymentMethodIconsBuilder {
        <PaymentMethodBasePaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodBasePaymentMethodIconsBuilder {
    card: Option<PaymentMethodBasePaymentMethodIconsCard>,
    square: Option<PaymentMethodBasePaymentMethodIconsSquare>,
}

impl PaymentMethodBasePaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodBasePaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodBasePaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodBasePaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodBasePaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodBasePaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodBasePaymentMethodIcons, BuildError> {
        Ok(PaymentMethodBasePaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
