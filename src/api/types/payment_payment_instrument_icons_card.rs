pub use crate::prelude::*;

/// The credit-card-proportioned tile (48x30).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentPaymentInstrumentIconsCard {
    /// The colorway for dark surfaces.
    #[serde(default)]
    pub dark: PaymentPaymentInstrumentIconsCardDark,
    /// The colorway for light surfaces.
    #[serde(default)]
    pub light: PaymentPaymentInstrumentIconsCardLight,
}

impl PaymentPaymentInstrumentIconsCard {
    pub fn builder() -> PaymentPaymentInstrumentIconsCardBuilder {
        <PaymentPaymentInstrumentIconsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPaymentInstrumentIconsCardBuilder {
    dark: Option<PaymentPaymentInstrumentIconsCardDark>,
    light: Option<PaymentPaymentInstrumentIconsCardLight>,
}

impl PaymentPaymentInstrumentIconsCardBuilder {
    pub fn dark(mut self, value: PaymentPaymentInstrumentIconsCardDark) -> Self {
        self.dark = Some(value);
        self
    }

    pub fn light(mut self, value: PaymentPaymentInstrumentIconsCardLight) -> Self {
        self.light = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentPaymentInstrumentIconsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dark`](PaymentPaymentInstrumentIconsCardBuilder::dark)
    /// - [`light`](PaymentPaymentInstrumentIconsCardBuilder::light)
    pub fn build(self) -> Result<PaymentPaymentInstrumentIconsCard, BuildError> {
        Ok(PaymentPaymentInstrumentIconsCard {
            dark: self.dark.ok_or_else(|| BuildError::missing_field("dark"))?,
            light: self
                .light
                .ok_or_else(|| BuildError::missing_field("light"))?,
        })
    }
}
