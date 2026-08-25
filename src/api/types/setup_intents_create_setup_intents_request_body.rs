pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateSetupIntentsRequestBody {
    CreateSetupIntentsRequestBodyConfirmationToken(CreateSetupIntentsRequestBodyConfirmationToken),

    CreateSetupIntentsRequestBodyPaymentMethodId(CreateSetupIntentsRequestBodyPaymentMethodId),
}

impl CreateSetupIntentsRequestBody {
    pub fn is_create_setup_intents_request_body_confirmation_token(&self) -> bool {
        matches!(
            self,
            Self::CreateSetupIntentsRequestBodyConfirmationToken(_)
        )
    }

    pub fn is_create_setup_intents_request_body_payment_method_id(&self) -> bool {
        matches!(self, Self::CreateSetupIntentsRequestBodyPaymentMethodId(_))
    }

    pub fn as_create_setup_intents_request_body_confirmation_token(
        &self,
    ) -> Option<&CreateSetupIntentsRequestBodyConfirmationToken> {
        match self {
            Self::CreateSetupIntentsRequestBodyConfirmationToken(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_setup_intents_request_body_confirmation_token(
        self,
    ) -> Option<CreateSetupIntentsRequestBodyConfirmationToken> {
        match self {
            Self::CreateSetupIntentsRequestBodyConfirmationToken(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_setup_intents_request_body_payment_method_id(
        &self,
    ) -> Option<&CreateSetupIntentsRequestBodyPaymentMethodId> {
        match self {
            Self::CreateSetupIntentsRequestBodyPaymentMethodId(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_setup_intents_request_body_payment_method_id(
        self,
    ) -> Option<CreateSetupIntentsRequestBodyPaymentMethodId> {
        match self {
            Self::CreateSetupIntentsRequestBodyPaymentMethodId(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateSetupIntentsRequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateSetupIntentsRequestBodyConfirmationToken(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreateSetupIntentsRequestBodyPaymentMethodId(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
