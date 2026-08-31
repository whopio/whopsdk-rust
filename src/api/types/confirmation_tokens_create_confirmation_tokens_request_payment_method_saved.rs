pub use crate::prelude::*;

/// Category `saved` only. Names one of the buyer's own stored payment methods. Requires a buyer credential — the wallet read is scoped to that account, so another user's id reads as not found.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConfirmationTokensRequestPaymentMethodSaved {
    /// The stored payment method to charge — a payment method id from GET /payment_methods.
    #[serde(default)]
    pub payment_method: String,
}

impl CreateConfirmationTokensRequestPaymentMethodSaved {
    pub fn builder() -> CreateConfirmationTokensRequestPaymentMethodSavedBuilder {
        <CreateConfirmationTokensRequestPaymentMethodSavedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestPaymentMethodSavedBuilder {
    payment_method: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethodSavedBuilder {
    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestPaymentMethodSaved`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_method`](CreateConfirmationTokensRequestPaymentMethodSavedBuilder::payment_method)
    pub fn build(self) -> Result<CreateConfirmationTokensRequestPaymentMethodSaved, BuildError> {
        Ok(CreateConfirmationTokensRequestPaymentMethodSaved {
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
        })
    }
}
