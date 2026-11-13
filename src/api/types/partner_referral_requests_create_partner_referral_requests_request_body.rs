pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreatePartnerReferralRequestsRequestBody {
    CreatePartnerReferralRequestsRequestBodyAccountId(
        CreatePartnerReferralRequestsRequestBodyAccountId,
    ),

    CreatePartnerReferralRequestsRequestBodyAccountUrl(
        CreatePartnerReferralRequestsRequestBodyAccountUrl,
    ),

    CreatePartnerReferralRequestsRequestBodyTargetUserId(
        CreatePartnerReferralRequestsRequestBodyTargetUserId,
    ),

    CreatePartnerReferralRequestsRequestBodyTargetUsername(
        CreatePartnerReferralRequestsRequestBodyTargetUsername,
    ),

    CreatePartnerReferralRequestsRequestBodyTargetEmail(
        CreatePartnerReferralRequestsRequestBodyTargetEmail,
    ),

    CreatePartnerReferralRequestsRequestBodyCode(CreatePartnerReferralRequestsRequestBodyCode),
}

impl CreatePartnerReferralRequestsRequestBody {
    pub fn is_create_partner_referral_requests_request_body_account_id(&self) -> bool {
        matches!(
            self,
            Self::CreatePartnerReferralRequestsRequestBodyAccountId(_)
        )
    }

    pub fn is_create_partner_referral_requests_request_body_account_url(&self) -> bool {
        matches!(
            self,
            Self::CreatePartnerReferralRequestsRequestBodyAccountUrl(_)
        )
    }

    pub fn is_create_partner_referral_requests_request_body_target_user_id(&self) -> bool {
        matches!(
            self,
            Self::CreatePartnerReferralRequestsRequestBodyTargetUserId(_)
        )
    }

    pub fn is_create_partner_referral_requests_request_body_target_username(&self) -> bool {
        matches!(
            self,
            Self::CreatePartnerReferralRequestsRequestBodyTargetUsername(_)
        )
    }

    pub fn is_create_partner_referral_requests_request_body_target_email(&self) -> bool {
        matches!(
            self,
            Self::CreatePartnerReferralRequestsRequestBodyTargetEmail(_)
        )
    }

    pub fn is_create_partner_referral_requests_request_body_code(&self) -> bool {
        matches!(self, Self::CreatePartnerReferralRequestsRequestBodyCode(_))
    }

    pub fn as_create_partner_referral_requests_request_body_account_id(
        &self,
    ) -> Option<&CreatePartnerReferralRequestsRequestBodyAccountId> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyAccountId(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_partner_referral_requests_request_body_account_id(
        self,
    ) -> Option<CreatePartnerReferralRequestsRequestBodyAccountId> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyAccountId(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_partner_referral_requests_request_body_account_url(
        &self,
    ) -> Option<&CreatePartnerReferralRequestsRequestBodyAccountUrl> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyAccountUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_partner_referral_requests_request_body_account_url(
        self,
    ) -> Option<CreatePartnerReferralRequestsRequestBodyAccountUrl> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyAccountUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_partner_referral_requests_request_body_target_user_id(
        &self,
    ) -> Option<&CreatePartnerReferralRequestsRequestBodyTargetUserId> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyTargetUserId(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_partner_referral_requests_request_body_target_user_id(
        self,
    ) -> Option<CreatePartnerReferralRequestsRequestBodyTargetUserId> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyTargetUserId(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_partner_referral_requests_request_body_target_username(
        &self,
    ) -> Option<&CreatePartnerReferralRequestsRequestBodyTargetUsername> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyTargetUsername(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_partner_referral_requests_request_body_target_username(
        self,
    ) -> Option<CreatePartnerReferralRequestsRequestBodyTargetUsername> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyTargetUsername(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_partner_referral_requests_request_body_target_email(
        &self,
    ) -> Option<&CreatePartnerReferralRequestsRequestBodyTargetEmail> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyTargetEmail(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_partner_referral_requests_request_body_target_email(
        self,
    ) -> Option<CreatePartnerReferralRequestsRequestBodyTargetEmail> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyTargetEmail(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_partner_referral_requests_request_body_code(
        &self,
    ) -> Option<&CreatePartnerReferralRequestsRequestBodyCode> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyCode(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_partner_referral_requests_request_body_code(
        self,
    ) -> Option<CreatePartnerReferralRequestsRequestBodyCode> {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyCode(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePartnerReferralRequestsRequestBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatePartnerReferralRequestsRequestBodyAccountId(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePartnerReferralRequestsRequestBodyAccountUrl(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePartnerReferralRequestsRequestBodyTargetUserId(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePartnerReferralRequestsRequestBodyTargetUsername(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePartnerReferralRequestsRequestBodyTargetEmail(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::CreatePartnerReferralRequestsRequestBodyCode(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
