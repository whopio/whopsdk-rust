pub use crate::prelude::*;

/// The square tile (32x32).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemUsBankAccountPaymentMethodIconsSquare {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareLight,
}

impl PaymentMethodListItemUsBankAccountPaymentMethodIconsSquare {
    pub fn builder() -> PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareBuilder {
        <PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareBuilder {
    dark: Option<PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDark>,
    light: Option<PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareLight>,
}

impl PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareBuilder {
    pub fn dark(
        mut self,
        value: PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareDark,
    ) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(
        mut self,
        value: PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareLight,
    ) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemUsBankAccountPaymentMethodIconsSquare`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareBuilder::dark)
    /// - [`light`](PaymentMethodListItemUsBankAccountPaymentMethodIconsSquareBuilder::light)
    pub fn build(
        self,
    ) -> Result<PaymentMethodListItemUsBankAccountPaymentMethodIconsSquare, BuildError> {
        Ok(PaymentMethodListItemUsBankAccountPaymentMethodIconsSquare {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
