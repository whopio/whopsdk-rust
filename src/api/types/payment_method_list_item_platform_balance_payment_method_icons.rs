pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodListItemPlatformBalancePaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodListItemPlatformBalancePaymentMethodIconsSquare,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodIcons {
    pub fn builder() -> PaymentMethodListItemPlatformBalancePaymentMethodIconsBuilder {
        <PaymentMethodListItemPlatformBalancePaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemPlatformBalancePaymentMethodIconsBuilder {
    card: Option<PaymentMethodListItemPlatformBalancePaymentMethodIconsCard>,
    square: Option<PaymentMethodListItemPlatformBalancePaymentMethodIconsSquare>,
}

impl PaymentMethodListItemPlatformBalancePaymentMethodIconsBuilder {
    pub fn card(
        mut self,
        value: PaymentMethodListItemPlatformBalancePaymentMethodIconsCard,
    ) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(
        mut self,
        value: PaymentMethodListItemPlatformBalancePaymentMethodIconsSquare,
    ) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemPlatformBalancePaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodListItemPlatformBalancePaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodListItemPlatformBalancePaymentMethodIconsBuilder::square)
    pub fn build(
        self,
    ) -> Result<PaymentMethodListItemPlatformBalancePaymentMethodIcons, BuildError> {
        Ok(PaymentMethodListItemPlatformBalancePaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
