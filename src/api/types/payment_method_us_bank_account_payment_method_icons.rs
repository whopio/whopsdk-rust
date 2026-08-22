pub use crate::prelude::*;

/// Every rendition of the icon to display this payment method with. A saved card carries its brand's icon (Visa, Mastercard, ...) rather than the generic card art.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodUsBankAccountPaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodUsBankAccountPaymentMethodIconsCard,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodUsBankAccountPaymentMethodIconsSquare,
}

impl PaymentMethodUsBankAccountPaymentMethodIcons {
    pub fn builder() -> PaymentMethodUsBankAccountPaymentMethodIconsBuilder {
        <PaymentMethodUsBankAccountPaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodUsBankAccountPaymentMethodIconsBuilder {
    card: Option<PaymentMethodUsBankAccountPaymentMethodIconsCard>,
    square: Option<PaymentMethodUsBankAccountPaymentMethodIconsSquare>,
}

impl PaymentMethodUsBankAccountPaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodUsBankAccountPaymentMethodIconsCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodUsBankAccountPaymentMethodIconsSquare) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodUsBankAccountPaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodUsBankAccountPaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodUsBankAccountPaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodUsBankAccountPaymentMethodIcons, BuildError> {
        Ok(PaymentMethodUsBankAccountPaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
