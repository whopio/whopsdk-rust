pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentLastPaymentError {
    /// A machine-readable classification of the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The reason the payment was declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_code: Option<PaymentLastPaymentErrorDeclineCode>,
    /// A human-readable explanation of the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl PaymentLastPaymentError {
    pub fn builder() -> PaymentLastPaymentErrorBuilder {
        <PaymentLastPaymentErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentLastPaymentErrorBuilder {
    code: Option<String>,
    decline_code: Option<PaymentLastPaymentErrorDeclineCode>,
    message: Option<String>,
}

impl PaymentLastPaymentErrorBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn decline_code(mut self, value: PaymentLastPaymentErrorDeclineCode) -> Self {
        self.decline_code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentLastPaymentError`].
    pub fn build(self) -> Result<PaymentLastPaymentError, BuildError> {
        Ok(PaymentLastPaymentError {
            code: self.code,
            decline_code: self.decline_code,
            message: self.message,
        })
    }
}
