pub use crate::prelude::*;

/// The iDEAL-specific details for this payment method, including bank name and BIC.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemIdealPaymentMethodIdeal {
    /// The name of the customer's bank used for the iDEAL transaction. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
    /// The Bank Identifier Code (BIC/SWIFT) of the customer's bank. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
}

impl PaymentMethodListItemIdealPaymentMethodIdeal {
    pub fn builder() -> PaymentMethodListItemIdealPaymentMethodIdealBuilder {
        <PaymentMethodListItemIdealPaymentMethodIdealBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemIdealPaymentMethodIdealBuilder {
    bank: Option<String>,
    bic: Option<String>,
}

impl PaymentMethodListItemIdealPaymentMethodIdealBuilder {
    pub fn bank(mut self, value: impl Into<String>) -> Self {
        self.bank = Some(value.into());
        self
    }

    pub fn bic(mut self, value: impl Into<String>) -> Self {
        self.bic = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemIdealPaymentMethodIdeal`].
    pub fn build(self) -> Result<PaymentMethodListItemIdealPaymentMethodIdeal, BuildError> {
        Ok(PaymentMethodListItemIdealPaymentMethodIdeal {
            bank: self.bank,
            bic: self.bic,
        })
    }
}
