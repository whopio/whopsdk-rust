pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateDepositsRequestDestination {
    String(String),

    CreateDepositsRequestDestinationAccountId(CreateDepositsRequestDestinationAccountId),
}

impl CreateDepositsRequestDestination {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_create_deposits_request_destination_account_id(&self) -> bool {
        matches!(self, Self::CreateDepositsRequestDestinationAccountId(_))
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_deposits_request_destination_account_id(
        &self,
    ) -> Option<&CreateDepositsRequestDestinationAccountId> {
        match self {
            Self::CreateDepositsRequestDestinationAccountId(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_deposits_request_destination_account_id(
        self,
    ) -> Option<CreateDepositsRequestDestinationAccountId> {
        match self {
            Self::CreateDepositsRequestDestinationAccountId(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateDepositsRequestDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::CreateDepositsRequestDestinationAccountId(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
