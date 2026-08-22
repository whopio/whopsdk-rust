pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodPlatformBalancePaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodPlatformBalancePaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodPlatformBalancePaymentMethodIconsSquare,
}

impl PaymentMethodPlatformBalancePaymentMethodIcons {
    pub fn builder() -> PaymentMethodPlatformBalancePaymentMethodIconsBuilder {
        <PaymentMethodPlatformBalancePaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodPlatformBalancePaymentMethodIconsBuilder {
    card: Option<PaymentMethodPlatformBalancePaymentMethodIconsCard>,
    square: Option<PaymentMethodPlatformBalancePaymentMethodIconsSquare>,
}

impl PaymentMethodPlatformBalancePaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodPlatformBalancePaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodPlatformBalancePaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodPlatformBalancePaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodPlatformBalancePaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodPlatformBalancePaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodPlatformBalancePaymentMethodIcons, BuildError> {
        Ok(PaymentMethodPlatformBalancePaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
