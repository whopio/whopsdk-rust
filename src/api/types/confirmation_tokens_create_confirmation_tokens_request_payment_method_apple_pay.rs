pub use crate::prelude::*;

/// Type `apple_pay` (category `wallet`) only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConfirmationTokensRequestPaymentMethodApplePay {
    /// The merchant identifier the Apple Pay sheet validated with — the same hostname-matched value from the type's `merchants` list the session ceremony used. Apple encrypts the wallet token for the certificate attached to this exact identifier, so the charge needs it to decrypt. Omit it when the ceremony omitted it; must be one the account has registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_identifier: Option<String>,
    /// The Basis Theory token intent the Apple Pay sheet flow vaulted the raw wallet token into.
    #[serde(default)]
    pub token_intent: String,
}

impl CreateConfirmationTokensRequestPaymentMethodApplePay {
    pub fn builder() -> CreateConfirmationTokensRequestPaymentMethodApplePayBuilder {
        <CreateConfirmationTokensRequestPaymentMethodApplePayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestPaymentMethodApplePayBuilder {
    merchant_identifier: Option<String>,
    token_intent: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethodApplePayBuilder {
    pub fn merchant_identifier(mut self, value: impl Into<String>) -> Self {
        self.merchant_identifier = Some(value.into());
        self
    }

    pub fn token_intent(mut self, value: impl Into<String>) -> Self {
        self.token_intent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestPaymentMethodApplePay`].
    /// This method will fail if any of the following fields are not set:
    /// - [`token_intent`](CreateConfirmationTokensRequestPaymentMethodApplePayBuilder::token_intent)
    pub fn build(self) -> Result<CreateConfirmationTokensRequestPaymentMethodApplePay, BuildError> {
        Ok(CreateConfirmationTokensRequestPaymentMethodApplePay {
            merchant_identifier: self.merchant_identifier,
            token_intent: self
                .token_intent
                .ok_or_else(|| BuildError::missing_field("token_intent"))?,
        })
    }
}
