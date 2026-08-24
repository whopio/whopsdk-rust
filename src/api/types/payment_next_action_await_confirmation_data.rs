pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentNextActionAwaitConfirmationData {
    /// When the confirmation window closes, as an ISO 8601 timestamp. A payment still unconfirmed by then will not succeed — watch `status` for the failed attempt.
    #[serde(default)]
    pub expires_at: String,
}

impl PaymentNextActionAwaitConfirmationData {
    pub fn builder() -> PaymentNextActionAwaitConfirmationDataBuilder {
        <PaymentNextActionAwaitConfirmationDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentNextActionAwaitConfirmationDataBuilder {
    expires_at: Option<String>,
}

impl PaymentNextActionAwaitConfirmationDataBuilder {
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentNextActionAwaitConfirmationData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](PaymentNextActionAwaitConfirmationDataBuilder::expires_at)
    pub fn build(self) -> Result<PaymentNextActionAwaitConfirmationData, BuildError> {
        Ok(PaymentNextActionAwaitConfirmationData {
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
