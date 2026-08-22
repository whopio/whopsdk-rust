pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemIdealPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemIdealPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemIdealPaymentMethodIconsSquareLight,
}

impl PaymentMethodListItemIdealPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodListItemIdealPaymentMethodIconsSquareBuilder {
        <PaymentMethodListItemIdealPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemIdealPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodListItemIdealPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodListItemIdealPaymentMethodIconsSquareLight>,
}

impl PaymentMethodListItemIdealPaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodListItemIdealPaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodListItemIdealPaymentMethodIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemIdealPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemIdealPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodListItemIdealPaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodListItemIdealPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodListItemIdealPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
