pub use crate::prelude::*;

/// The payment that settled this invoice. Null while the invoice is unpaid, when the invoice was marked paid manually, and on a subscription renewal invoice, where the settling payment cannot yet be identified.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicePayment {
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
}

impl InvoicePayment {
    pub fn builder() -> InvoicePaymentBuilder {
        <InvoicePaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicePaymentBuilder {
    id: Option<String>,
}

impl InvoicePaymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoicePayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoicePaymentBuilder::id)
    pub fn build(self) -> Result<InvoicePayment, BuildError> {
        Ok(InvoicePayment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
