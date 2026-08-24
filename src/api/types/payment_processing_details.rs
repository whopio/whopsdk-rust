pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentProcessingDetails {
    /// When the payment is expected to settle, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_by: Option<String>,
}

impl PaymentProcessingDetails {
    pub fn builder() -> PaymentProcessingDetailsBuilder {
        <PaymentProcessingDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentProcessingDetailsBuilder {
    expected_by: Option<String>,
}

impl PaymentProcessingDetailsBuilder {
    pub fn expected_by(mut self, value: impl Into<String>) -> Self {
        self.expected_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentProcessingDetails`].
    pub fn build(self) -> Result<PaymentProcessingDetails, BuildError> {
        Ok(PaymentProcessingDetails {
            expected_by: self.expected_by,
        })
    }
}
