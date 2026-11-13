pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChargeInvoicesRequest {
    /// The saved payment method to charge, prefixed `payt_`. It must belong to the invoice's customer and be chargeable without the customer present. Omit it to charge the invoice's own `payment_method_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
}

impl ChargeInvoicesRequest {
    pub fn builder() -> ChargeInvoicesRequestBuilder {
        <ChargeInvoicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChargeInvoicesRequestBuilder {
    payment_method_id: Option<String>,
}

impl ChargeInvoicesRequestBuilder {
    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChargeInvoicesRequest`].
    pub fn build(self) -> Result<ChargeInvoicesRequest, BuildError> {
        Ok(ChargeInvoicesRequest {
            payment_method_id: self.payment_method_id,
        })
    }
}
