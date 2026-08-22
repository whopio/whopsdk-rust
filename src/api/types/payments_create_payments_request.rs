pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePaymentsRequest {
    CreatePaymentsRequestZero(CreatePaymentsRequestZero),

    CreatePaymentsRequestOne(CreatePaymentsRequestOne),

    CreatePaymentsRequestTwo(CreatePaymentsRequestTwo),

    CreatePaymentsRequestThree(CreatePaymentsRequestThree),
}

impl CreatePaymentsRequest {
    pub fn is_create_payments_request_zero(&self) -> bool {
        matches!(self, Self::CreatePaymentsRequestZero(_))
    }

    pub fn is_create_payments_request_one(&self) -> bool {
        matches!(self, Self::CreatePaymentsRequestOne(_))
    }

    pub fn is_create_payments_request_two(&self) -> bool {
        matches!(self, Self::CreatePaymentsRequestTwo(_))
    }

    pub fn is_create_payments_request_three(&self) -> bool {
        matches!(self, Self::CreatePaymentsRequestThree(_))
    }

    pub fn as_create_payments_request_zero(&self) -> Option<&CreatePaymentsRequestZero> {
        match self {
            Self::CreatePaymentsRequestZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_payments_request_zero(self) -> Option<CreatePaymentsRequestZero> {
        match self {
            Self::CreatePaymentsRequestZero(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_payments_request_one(&self) -> Option<&CreatePaymentsRequestOne> {
        match self {
            Self::CreatePaymentsRequestOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_payments_request_one(self) -> Option<CreatePaymentsRequestOne> {
        match self {
            Self::CreatePaymentsRequestOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_payments_request_two(&self) -> Option<&CreatePaymentsRequestTwo> {
        match self {
            Self::CreatePaymentsRequestTwo(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_payments_request_two(self) -> Option<CreatePaymentsRequestTwo> {
        match self {
            Self::CreatePaymentsRequestTwo(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_payments_request_three(&self) -> Option<&CreatePaymentsRequestThree> {
        match self {
            Self::CreatePaymentsRequestThree(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_payments_request_three(self) -> Option<CreatePaymentsRequestThree> {
        match self {
            Self::CreatePaymentsRequestThree(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePaymentsRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatePaymentsRequestZero(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePaymentsRequestOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePaymentsRequestTwo(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePaymentsRequestThree(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
