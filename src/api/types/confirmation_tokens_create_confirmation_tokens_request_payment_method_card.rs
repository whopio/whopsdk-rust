pub use crate::prelude::*;

/// Category `card` only. Exactly one of `token` or `token_intent`; display fields ride alongside.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConfirmationTokensRequestPaymentMethodCard {
    /// Display-safe card brand from the collection surface, e.g. `visa`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    /// Display-safe last four digits from the collection surface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// An element-assembled, expiring Basis Theory token. Provide this or token_intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// A Basis Theory token intent. Provide this or token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_intent: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethodCard {
    pub fn builder() -> CreateConfirmationTokensRequestPaymentMethodCardBuilder {
        <CreateConfirmationTokensRequestPaymentMethodCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestPaymentMethodCardBuilder {
    brand: Option<String>,
    last4: Option<String>,
    token: Option<String>,
    token_intent: Option<String>,
}

impl CreateConfirmationTokensRequestPaymentMethodCardBuilder {
    pub fn brand(mut self, value: impl Into<String>) -> Self {
        self.brand = Some(value.into());
        self
    }

    pub fn last4(mut self, value: impl Into<String>) -> Self {
        self.last4 = Some(value.into());
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    pub fn token_intent(mut self, value: impl Into<String>) -> Self {
        self.token_intent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestPaymentMethodCard`].
    pub fn build(self) -> Result<CreateConfirmationTokensRequestPaymentMethodCard, BuildError> {
        Ok(CreateConfirmationTokensRequestPaymentMethodCard {
            brand: self.brand,
            last4: self.last4,
            token: self.token,
            token_intent: self.token_intent,
        })
    }
}
