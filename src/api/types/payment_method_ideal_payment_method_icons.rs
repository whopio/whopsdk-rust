pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodIdealPaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodIdealPaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodIdealPaymentMethodIconsSquare,
}

impl PaymentMethodIdealPaymentMethodIcons {
    pub fn builder() -> PaymentMethodIdealPaymentMethodIconsBuilder {
        <PaymentMethodIdealPaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodIdealPaymentMethodIconsBuilder {
    card: Option<PaymentMethodIdealPaymentMethodIconsCard>,
    square: Option<PaymentMethodIdealPaymentMethodIconsSquare>,
}

impl PaymentMethodIdealPaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodIdealPaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodIdealPaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodIdealPaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodIdealPaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodIdealPaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodIdealPaymentMethodIcons, BuildError> {
        Ok(PaymentMethodIdealPaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
