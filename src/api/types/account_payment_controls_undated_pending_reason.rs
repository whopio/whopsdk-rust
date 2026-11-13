pub use crate::prelude::*;

/// Why pending funds without a settlement date aren't moving yet. `kyc_incomplete`, `pending_information_request`, and `update_payout_profile` are things the merchant can act on. `compliance_review` means a verification the merchant already submitted is being reviewed, so these funds stay pending until it clears. `withdrawals_disabled` means Whop has blocked withdrawals, so these funds cannot become available. `null` when there's no reason to show — still clearing, or held for a reason that isn't named here.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountPaymentControlsUndatedPendingReason {
    KycIncomplete,
    PendingInformationRequest,
    UpdatePayoutProfile,
    ComplianceReview,
    WithdrawalsDisabled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountPaymentControlsUndatedPendingReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::KycIncomplete => serializer.serialize_str("kyc_incomplete"),
            Self::PendingInformationRequest => {
                serializer.serialize_str("pending_information_request")
            }
            Self::UpdatePayoutProfile => serializer.serialize_str("update_payout_profile"),
            Self::ComplianceReview => serializer.serialize_str("compliance_review"),
            Self::WithdrawalsDisabled => serializer.serialize_str("withdrawals_disabled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountPaymentControlsUndatedPendingReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "kyc_incomplete" => Ok(Self::KycIncomplete),
            "pending_information_request" => Ok(Self::PendingInformationRequest),
            "update_payout_profile" => Ok(Self::UpdatePayoutProfile),
            "compliance_review" => Ok(Self::ComplianceReview),
            "withdrawals_disabled" => Ok(Self::WithdrawalsDisabled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountPaymentControlsUndatedPendingReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KycIncomplete => write!(f, "kyc_incomplete"),
            Self::PendingInformationRequest => write!(f, "pending_information_request"),
            Self::UpdatePayoutProfile => write!(f, "update_payout_profile"),
            Self::ComplianceReview => write!(f, "compliance_review"),
            Self::WithdrawalsDisabled => write!(f, "withdrawals_disabled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
