pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentRequiredErrorBody {
    pub error: PaymentRequiredErrorBodyError,
}

impl PaymentRequiredErrorBody {
    pub fn builder() -> PaymentRequiredErrorBodyBuilder {
        <PaymentRequiredErrorBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRequiredErrorBodyBuilder {
    error: Option<PaymentRequiredErrorBodyError>,
}

impl PaymentRequiredErrorBodyBuilder {
    pub fn error(mut self, value: PaymentRequiredErrorBodyError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentRequiredErrorBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](PaymentRequiredErrorBodyBuilder::error)
    pub fn build(self) -> Result<PaymentRequiredErrorBody, BuildError> {
        Ok(PaymentRequiredErrorBody {
            error: self
                .error
                .ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
