pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemIdealPaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodListItemIdealPaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodListItemIdealPaymentMethodIconsSquare,
}

impl PaymentMethodListItemIdealPaymentMethodIcons {
    pub fn builder() -> PaymentMethodListItemIdealPaymentMethodIconsBuilder {
        <PaymentMethodListItemIdealPaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemIdealPaymentMethodIconsBuilder {
    card: Option<PaymentMethodListItemIdealPaymentMethodIconsCard>,
    square: Option<PaymentMethodListItemIdealPaymentMethodIconsSquare>,
}

impl PaymentMethodListItemIdealPaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodListItemIdealPaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodListItemIdealPaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemIdealPaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodListItemIdealPaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodListItemIdealPaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodListItemIdealPaymentMethodIcons, BuildError> {
        Ok(PaymentMethodListItemIdealPaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
