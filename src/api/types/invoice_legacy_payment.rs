pub use crate::prelude::*;

/// The payment that settled this invoice. Null while the invoice is unpaid, when the invoice was marked paid manually, and on a subscription renewal invoice, where the settling payment cannot yet be identified.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceLegacyPayment {
    /// The unique identifier for the payment.
    #[serde(default)]
    pub id: String,
}

impl InvoiceLegacyPayment {
    pub fn builder() -> InvoiceLegacyPaymentBuilder {
        <InvoiceLegacyPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceLegacyPaymentBuilder {
    id: Option<String>,
}

impl InvoiceLegacyPaymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceLegacyPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoiceLegacyPaymentBuilder::id)
    pub fn build(self) -> Result<InvoiceLegacyPayment, BuildError> {
        Ok(InvoiceLegacyPayment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
