pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateVerificationsRequestBody {
    UpdateVerificationsRequestBodyPersonalAddress(UpdateVerificationsRequestBodyPersonalAddress),

    UpdateVerificationsRequestBodyBusinessAddress(UpdateVerificationsRequestBodyBusinessAddress),
}

impl UpdateVerificationsRequestBody {
    pub fn is_update_verifications_request_body_personal_address(&self) -> bool {
        matches!(self, Self::UpdateVerificationsRequestBodyPersonalAddress(_))
    }

    pub fn is_update_verifications_request_body_business_address(&self) -> bool {
        matches!(self, Self::UpdateVerificationsRequestBodyBusinessAddress(_))
    }

    pub fn as_update_verifications_request_body_personal_address(
        &self,
    ) -> Option<&UpdateVerificationsRequestBodyPersonalAddress> {
        match self {
            Self::UpdateVerificationsRequestBodyPersonalAddress(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_verifications_request_body_personal_address(
        self,
    ) -> Option<UpdateVerificationsRequestBodyPersonalAddress> {
        match self {
            Self::UpdateVerificationsRequestBodyPersonalAddress(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_update_verifications_request_body_business_address(
        &self,
    ) -> Option<&UpdateVerificationsRequestBodyBusinessAddress> {
        match self {
            Self::UpdateVerificationsRequestBodyBusinessAddress(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_verifications_request_body_business_address(
        self,
    ) -> Option<UpdateVerificationsRequestBodyBusinessAddress> {
        match self {
            Self::UpdateVerificationsRequestBodyBusinessAddress(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdateVerificationsRequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdateVerificationsRequestBodyPersonalAddress(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::UpdateVerificationsRequestBodyBusinessAddress(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
