pub use crate::prelude::*;

/// The iDEAL-specific details for this payment method, including bank name and BIC.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodIdealPaymentMethodIdeal {
    /// The name of the customer's bank used for the iDEAL transaction. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
    /// The Bank Identifier Code (BIC/SWIFT) of the customer's bank. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
}

impl PaymentMethodIdealPaymentMethodIdeal {
    pub fn builder() -> PaymentMethodIdealPaymentMethodIdealBuilder {
        <PaymentMethodIdealPaymentMethodIdealBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodIdealPaymentMethodIdealBuilder {
    bank: Option<String>,
    bic: Option<String>,
}

impl PaymentMethodIdealPaymentMethodIdealBuilder {
    pub fn bank(mut self, value: impl Into<String>) -> Self {
        self.bank = Some(value.into());
        self
    }

    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodIdealPaymentMethodIdeal`].
    pub fn build(self) -> Result<PaymentMethodIdealPaymentMethodIdeal, BuildError> {
        Ok(PaymentMethodIdealPaymentMethodIdeal {
            bank: self.bank,
            bic: self.bic,
        })
    }
}
