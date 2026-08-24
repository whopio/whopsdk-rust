pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodPlatformBalancePaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodPlatformBalancePaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodPlatformBalancePaymentMethodIconsSquareLight,
}

impl PaymentMethodPlatformBalancePaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodPlatformBalancePaymentMethodIconsSquareBuilder {
        <PaymentMethodPlatformBalancePaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodPlatformBalancePaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodPlatformBalancePaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodPlatformBalancePaymentMethodIconsSquareLight>,
}

impl PaymentMethodPlatformBalancePaymentMethodIconsSquareBuilder {
    pub fn dark(mut self, value: PaymentMethodPlatformBalancePaymentMethodIconsSquareDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(
        mut self,
        value: PaymentMethodPlatformBalancePaymentMethodIconsSquareLight,
    ) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodPlatformBalancePaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodPlatformBalancePaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodPlatformBalancePaymentMethodIconsSquareBuilder::light)
    pub fn build(self) -> Result<PaymentMethodPlatformBalancePaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodPlatformBalancePaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
