pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentInstrument {
    /// Card payments only: the card's network and last four.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentInstrumentCard>,
    /// Buyer-facing instrument name — "Visa •••• 4242" when the card surfaced, else the method's own name ("Klarna").
    #[serde(default)]
    pub display_name: String,
    /// The standard icon set: square and card shapes, each in light and dark colorways.
    #[serde(default)]
    pub icons: PaymentMethodIcons,
    /// Installment methods only: how many payments the charge splits into. Data, not copy — compose and translate the label client-side.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub installment_count: Option<f64>,
    /// The payment method type identifier, e.g. `card`, `klarna`, `apple_pay`.
    #[serde(default)]
    pub payment_method_type: String,
}

impl PaymentInstrument {
    pub fn builder() -> PaymentInstrumentBuilder {
        <PaymentInstrumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentInstrumentBuilder {
    card: Option<PaymentInstrumentCard>,
    display_name: Option<String>,
    icons: Option<PaymentMethodIcons>,
    installment_count: Option<f64>,
    payment_method_type: Option<String>,
}

impl PaymentInstrumentBuilder {
    pub fn card(mut self, value: PaymentInstrumentCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn icons(mut self, value: PaymentMethodIcons) -> Self {
        self.icons = Some(value);
        self
    }

    pub fn installment_count(mut self, value: f64) -> Self {
        self.installment_count = Some(value);
        self
    }

    pub fn payment_method_type(mut self, value: impl Into<String>) -> Self {
        self.payment_method_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentInstrument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](PaymentInstrumentBuilder::display_name)
    /// - [`icons`](PaymentInstrumentBuilder::icons)
    /// - [`payment_method_type`](PaymentInstrumentBuilder::payment_method_type)
    pub fn build(self) -> Result<PaymentInstrument, BuildError> {
        Ok(PaymentInstrument {
            card: self.card,
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            icons: self
                .icons
                .ok_or_else(|| BuildError::missing_field("icons"))?,
            installment_count: self.installment_count,
            payment_method_type: self
                .payment_method_type
                .ok_or_else(|| BuildError::missing_field("payment_method_type"))?,
        })
    }
}
