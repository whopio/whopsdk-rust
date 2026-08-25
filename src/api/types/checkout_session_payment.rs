pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionPayment {
    /// The payment created by the confirm, prefixed `pay_`.
    #[serde(default)]
    pub id: String,
    /// Where the payment stands, and the only honest reading of a completed checkout's outcome. `requires_action` — a step remains and `next_action` carries it. `processing` — accepted and settling (or deciding); hold. `succeeded` — the money moved. `failed` — the charge died (declined, expired, voided): the checkout did not go through, whatever the session's own `status` says, and the buyer needs a fresh checkout to try again.
    pub status: CheckoutSessionPaymentStatus,
}

impl CheckoutSessionPayment {
    pub fn builder() -> CheckoutSessionPaymentBuilder {
        <CheckoutSessionPaymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionPaymentBuilder {
    id: Option<String>,
    status: Option<CheckoutSessionPaymentStatus>,
}

impl CheckoutSessionPaymentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn status(mut self, value: CheckoutSessionPaymentStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionPayment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CheckoutSessionPaymentBuilder::id)
    /// - [`status`](CheckoutSessionPaymentBuilder::status)
    pub fn build(self) -> Result<CheckoutSessionPayment, BuildError> {
        Ok(CheckoutSessionPayment {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
