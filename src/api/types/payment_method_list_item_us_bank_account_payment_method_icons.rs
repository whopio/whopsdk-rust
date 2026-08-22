pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemUsBankAccountPaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodListItemUsBankAccountPaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodListItemUsBankAccountPaymentMethodIconsSquare,
}

impl PaymentMethodListItemUsBankAccountPaymentMethodIcons {
    pub fn builder() -> PaymentMethodListItemUsBankAccountPaymentMethodIconsBuilder {
        <PaymentMethodListItemUsBankAccountPaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemUsBankAccountPaymentMethodIconsBuilder {
    card: Option<PaymentMethodListItemUsBankAccountPaymentMethodIconsCard>,
    square: Option<PaymentMethodListItemUsBankAccountPaymentMethodIconsSquare>,
}

impl PaymentMethodListItemUsBankAccountPaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodListItemUsBankAccountPaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(
        mut self,
        value: PaymentMethodListItemUsBankAccountPaymentMethodIconsSquare,
    ) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemUsBankAccountPaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodListItemUsBankAccountPaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodListItemUsBankAccountPaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodListItemUsBankAccountPaymentMethodIcons, BuildError> {
        Ok(PaymentMethodListItemUsBankAccountPaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
