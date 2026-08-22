pub use crate::prelude::*;

/// The Cash App-specific details for this payment method, including cashtag and buyer ID.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodListItemCashappPaymentMethodCashapp {
    /// The unique and immutable identifier assigned by Cash App to the buyer. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    /// The public cashtag handle of the buyer on Cash App. Null if not available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtag: Option<String>,
}

impl PaymentMethodListItemCashappPaymentMethodCashapp {
    pub fn builder() -> PaymentMethodListItemCashappPaymentMethodCashappBuilder {
        <PaymentMethodListItemCashappPaymentMethodCashappBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodListItemCashappPaymentMethodCashappBuilder {
    buyer_id: Option<String>,
    cashtag: Option<String>,
}

impl PaymentMethodListItemCashappPaymentMethodCashappBuilder {
    pub fn buyer_id(mut self, value: impl Into<String>) -> Self {
        self.buyer_id = Some(value.into());
        self
    }

    pub fn cashtag(mut self, value: impl Into<String>) -> Self {
        self.cashtag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodListItemCashappPaymentMethodCashapp`].
    pub fn build(self) -> Result<PaymentMethodListItemCashappPaymentMethodCashapp, BuildError> {
        Ok(PaymentMethodListItemCashappPaymentMethodCashapp {
            buyer_id: self.buyer_id,
            cashtag: self.cashtag,
        })
    }
}
