pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemBasePaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemBasePaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemBasePaymentMethodIconsSquareLight,
}

impl PaymentMethodListItemBasePaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodListItemBasePaymentMethodIconsSquareBuilder {
        <PaymentMethodListItemBasePaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemBasePaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodListItemBasePaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodListItemBasePaymentMethodIconsSquareLight>,
}

impl PaymentMethodListItemBasePaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodListItemBasePaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodListItemBasePaymentMethodIconsSquareLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemBasePaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemBasePaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodListItemBasePaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodListItemBasePaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodListItemBasePaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
