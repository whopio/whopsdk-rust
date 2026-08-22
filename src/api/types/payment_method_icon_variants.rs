pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodIconVariants {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentMethodIconFiles,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentMethodIconFiles,
}

impl PaymentMethodIconVariants {
    pub fn builder() -> PaymentMethodIconVariantsBuilder {
        <PaymentMethodIconVariantsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodIconVariantsBuilder {
    dark: Option<PaymentMethodIconFiles>,
    light: Option<PaymentMethodIconFiles>,
}

impl PaymentMethodIconVariantsBuilder {
    pub fn dark(mut self, value: PaymentMethodIconFiles) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentMethodIconFiles) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodIconVariants`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentMethodIconVariantsBuilder::dark)
    /// - [`light`](PaymentMethodIconVariantsBuilder::light)
    pub fn build(self) -> Result<PaymentMethodIconVariants, BuildError> {
        Ok(PaymentMethodIconVariants {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
