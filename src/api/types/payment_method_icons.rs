pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodIcons {
    /// The credit-card-proportioned tile (48x30).
    #[serde(default)]
    pub card: PaymentMethodIconVariants,
    /// The square tile (32x32).
    #[serde(default)]
    pub square: PaymentMethodIconVariants,
}

impl PaymentMethodIcons {
    pub fn builder() -> PaymentMethodIconsBuilder {
        <PaymentMethodIconsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodIconsBuilder {
    card: Option<PaymentMethodIconVariants>,
    square: Option<PaymentMethodIconVariants>,
}

impl PaymentMethodIconsBuilder {
    pub fn card(mut self, value: PaymentMethodIconVariants) -> Self {
        self.card = Some(value);
        self
    }

    pub fn square(mut self, value: PaymentMethodIconVariants) -> Self {
        self.square = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodIcons`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card`](PaymentMethodIconsBuilder::card)
    /// - [`square`](PaymentMethodIconsBuilder::square)
    pub fn build(self) -> Result<PaymentMethodIcons, BuildError> {
        Ok(PaymentMethodIcons {
            card: self.card.ok_or_else(|| BuildError::missing_field("card"))?,
            square: self
                .square
                .ok_or_else(|| BuildError::missing_field("square"))?,
        })
    }
}
