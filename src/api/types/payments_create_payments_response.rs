pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePaymentsResponse {
    #[serde(flatten)]
    pub payment_fields: Payment,
    /// The credential the buyer's surface presents to poll this payment and set its return URL. Returned when a payment created from a confirmation token is created or retrieved by a caller with the payment:charge permission. Null for payments created from a stored payment method or callers without payment:charge. It unlocks this payment and nothing else; treat it like a password for that one attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

impl CreatePaymentsResponse {
    pub fn builder() -> CreatePaymentsResponseBuilder {
        <CreatePaymentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsResponseBuilder {
    payment_fields: Option<Payment>,
    client_secret: Option<String>,
}

impl CreatePaymentsResponseBuilder {
    pub fn payment_fields(mut self, value: Payment) -> Self {
        self.payment_fields = Some(value);
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_fields`](CreatePaymentsResponseBuilder::payment_fields)
    pub fn build(self) -> Result<CreatePaymentsResponse, BuildError> {
        Ok(CreatePaymentsResponse {
            payment_fields: self
                .payment_fields
                .ok_or_else(|| BuildError::missing_field("payment_fields"))?,
            client_secret: self.client_secret,
        })
    }
}
