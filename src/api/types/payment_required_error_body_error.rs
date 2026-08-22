pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentRequiredErrorBodyError {
    /// Hosted page where the account can deposit funds before retrying.
    #[serde(default)]
    pub deposit_url: String,
    #[serde(default)]
    pub message: String,
    pub r#type: PaymentRequiredErrorBodyErrorType,
}

impl PaymentRequiredErrorBodyError {
    pub fn builder() -> PaymentRequiredErrorBodyErrorBuilder {
        <PaymentRequiredErrorBodyErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRequiredErrorBodyErrorBuilder {
    deposit_url: Option<String>,
    message: Option<String>,
    r#type: Option<PaymentRequiredErrorBodyErrorType>,
}

impl PaymentRequiredErrorBodyErrorBuilder {
    pub fn deposit_url(mut self, value: impl Into<String>) -> Self {
        self.deposit_url = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PaymentRequiredErrorBodyErrorType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentRequiredErrorBodyError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deposit_url`](PaymentRequiredErrorBodyErrorBuilder::deposit_url)
    /// - [`message`](PaymentRequiredErrorBodyErrorBuilder::message)
    /// - [`r#type`](PaymentRequiredErrorBodyErrorBuilder::r#type)
    pub fn build(self) -> Result<PaymentRequiredErrorBodyError, BuildError> {
        Ok(PaymentRequiredErrorBodyError {
            deposit_url: self
                .deposit_url
                .ok_or_else(|| BuildError::missing_field("deposit_url"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
