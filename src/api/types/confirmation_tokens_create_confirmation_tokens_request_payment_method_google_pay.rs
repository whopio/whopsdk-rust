pub use crate::prelude::*;

/// Type `google_pay` (category `wallet`) only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConfirmationTokensRequestPaymentMethodGooglePay {
    /// The Basis Theory token intent the Google Pay sheet flow vaulted the raw wallet token into.
    #[serde(default)]
    pub token_intent: String,
}

impl CreateConfirmationTokensRequestPaymentMethodGooglePay {
    pub fn builder() -> CreateConfirmationTokensRequestPaymentMethodGooglePayBuilder {
        <CreateConfirmationTokensRequestPaymentMethodGooglePayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestPaymentMethodGooglePayBuilder {
    token_intent: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethodGooglePayBuilder {
    pub fn token_intent(mut self, value: impl Into<String>) -> Self {
        self.token_intent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestPaymentMethodGooglePay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token_intent`](CreateConfirmationTokensRequestPaymentMethodGooglePayBuilder::token_intent)
    pub fn build(
        self,
    ) -> Result<CreateConfirmationTokensRequestPaymentMethodGooglePay, BuildError> {
        Ok(CreateConfirmationTokensRequestPaymentMethodGooglePay {
            token_intent: self
                .token_intent
                .ok_or_else(|| BuildError::missing_field("token_intent"))?,
        })
    }
}
