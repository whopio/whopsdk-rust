pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemBasePaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodListItemBasePaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodListItemBasePaymentMethodIconsSquare,
}

impl PaymentMethodListItemBasePaymentMethodIcons {
    pub fn builder() -> PaymentMethodListItemBasePaymentMethodIconsBuilder {
        <PaymentMethodListItemBasePaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemBasePaymentMethodIconsBuilder {
    card: Option<PaymentMethodListItemBasePaymentMethodIconsCard>,
    square: Option<PaymentMethodListItemBasePaymentMethodIconsSquare>,
}

impl PaymentMethodListItemBasePaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodListItemBasePaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodListItemBasePaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemBasePaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodListItemBasePaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodListItemBasePaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodListItemBasePaymentMethodIcons, BuildError> {
        Ok(PaymentMethodListItemBasePaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
