pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateSetupIntentsRequest {
    CreateSetupIntentsRequestConfirmationToken(CreateSetupIntentsRequestConfirmationToken),

    CreateSetupIntentsRequestPaymentMethodId(CreateSetupIntentsRequestPaymentMethodId),
}

impl CreateSetupIntentsRequest {
    pub fn is_create_setup_intents_request_confirmation_token(&self) -> bool {
        matches!(self, Self::CreateSetupIntentsRequestConfirmationToken(_))
    }

    pub fn is_create_setup_intents_request_payment_method_id(&self) -> bool {
        matches!(self, Self::CreateSetupIntentsRequestPaymentMethodId(_))
    }

    pub fn as_create_setup_intents_request_confirmation_token(
        &self,
    ) -> Option<&CreateSetupIntentsRequestConfirmationToken> {
        match self {
            Self::CreateSetupIntentsRequestConfirmationToken(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_setup_intents_request_confirmation_token(
        self,
    ) -> Option<CreateSetupIntentsRequestConfirmationToken> {
        match self {
            Self::CreateSetupIntentsRequestConfirmationToken(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_setup_intents_request_payment_method_id(
        &self,
    ) -> Option<&CreateSetupIntentsRequestPaymentMethodId> {
        match self {
            Self::CreateSetupIntentsRequestPaymentMethodId(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_setup_intents_request_payment_method_id(
        self,
    ) -> Option<CreateSetupIntentsRequestPaymentMethodId> {
        match self {
            Self::CreateSetupIntentsRequestPaymentMethodId(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateSetupIntentsRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateSetupIntentsRequestConfirmationToken(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreateSetupIntentsRequestPaymentMethodId(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
