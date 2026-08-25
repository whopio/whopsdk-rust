pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePaymentsRequestBody {
    CreatePaymentsRequestBodyZero(CreatePaymentsRequestBodyZero),

    CreatePaymentsRequestBodyOne(CreatePaymentsRequestBodyOne),

    CreatePaymentsRequestBodyTwo(CreatePaymentsRequestBodyTwo),

    CreatePaymentsRequestBodyThree(CreatePaymentsRequestBodyThree),
}

impl CreatePaymentsRequestBody {
    pub fn is_create_payments_request_body_zero(&self) -> bool {
        matches!(self, Self::CreatePaymentsRequestBodyZero(_))
    }

    pub fn is_create_payments_request_body_one(&self) -> bool {
        matches!(self, Self::CreatePaymentsRequestBodyOne(_))
    }

    pub fn is_create_payments_request_body_two(&self) -> bool {
        matches!(self, Self::CreatePaymentsRequestBodyTwo(_))
    }

    pub fn is_create_payments_request_body_three(&self) -> bool {
        matches!(self, Self::CreatePaymentsRequestBodyThree(_))
    }

    pub fn as_create_payments_request_body_zero(&self) -> Option<&CreatePaymentsRequestBodyZero> {
        match self {
            Self::CreatePaymentsRequestBodyZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_payments_request_body_zero(self) -> Option<CreatePaymentsRequestBodyZero> {
        match self {
            Self::CreatePaymentsRequestBodyZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_payments_request_body_one(&self) -> Option<&CreatePaymentsRequestBodyOne> {
        match self {
            Self::CreatePaymentsRequestBodyOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_payments_request_body_one(self) -> Option<CreatePaymentsRequestBodyOne> {
        match self {
            Self::CreatePaymentsRequestBodyOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_payments_request_body_two(&self) -> Option<&CreatePaymentsRequestBodyTwo> {
        match self {
            Self::CreatePaymentsRequestBodyTwo(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_payments_request_body_two(self) -> Option<CreatePaymentsRequestBodyTwo> {
        match self {
            Self::CreatePaymentsRequestBodyTwo(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_payments_request_body_three(&self) -> Option<&CreatePaymentsRequestBodyThree> {
        match self {
            Self::CreatePaymentsRequestBodyThree(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_payments_request_body_three(self) -> Option<CreatePaymentsRequestBodyThree> {
        match self {
            Self::CreatePaymentsRequestBodyThree(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePaymentsRequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatePaymentsRequestBodyZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePaymentsRequestBodyOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePaymentsRequestBodyTwo(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePaymentsRequestBodyThree(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
