pub use crate::prelude::*;

/// Why a standard payout cannot move funds right now, or null when the cap is above 0.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMethodsResponseLimitsStandardErrorCode {
    AccountSuspended,
    BlockMoveMoneyOutBecauseClawback,
    SupportabilityCheckPayoutStatusHold,
    KycCompleted,
    RmiClear,
    IdentityRfiClear,
    EcommerceFulfillmentConnected,
    BlockMoveMoneyOut,
    BlockMoveMoneyOutSetByParent,
    CardUsageReviewPayoutStatusHold,
    NoAvailableBalance,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMethodsResponseLimitsStandardErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AccountSuspended => serializer.serialize_str("account_suspended"),
            Self::BlockMoveMoneyOutBecauseClawback => {
                serializer.serialize_str("block_move_money_out_because_clawback")
            }
            Self::SupportabilityCheckPayoutStatusHold => {
                serializer.serialize_str("supportability_check_payout_status_hold")
            }
            Self::KycCompleted => serializer.serialize_str("kyc_completed"),
            Self::RmiClear => serializer.serialize_str("rmi_clear"),
            Self::IdentityRfiClear => serializer.serialize_str("identity_rfi_clear"),
            Self::EcommerceFulfillmentConnected => {
                serializer.serialize_str("ecommerce_fulfillment_connected")
            }
            Self::BlockMoveMoneyOut => serializer.serialize_str("block_move_money_out"),
            Self::BlockMoveMoneyOutSetByParent => {
                serializer.serialize_str("block_move_money_out_set_by_parent")
            }
            Self::CardUsageReviewPayoutStatusHold => {
                serializer.serialize_str("card_usage_review_payout_status_hold")
            }
            Self::NoAvailableBalance => serializer.serialize_str("no_available_balance"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMethodsResponseLimitsStandardErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account_suspended" => Ok(Self::AccountSuspended),
            "block_move_money_out_because_clawback" => Ok(Self::BlockMoveMoneyOutBecauseClawback),
            "supportability_check_payout_status_hold" => {
                Ok(Self::SupportabilityCheckPayoutStatusHold)
            }
            "kyc_completed" => Ok(Self::KycCompleted),
            "rmi_clear" => Ok(Self::RmiClear),
            "identity_rfi_clear" => Ok(Self::IdentityRfiClear),
            "ecommerce_fulfillment_connected" => Ok(Self::EcommerceFulfillmentConnected),
            "block_move_money_out" => Ok(Self::BlockMoveMoneyOut),
            "block_move_money_out_set_by_parent" => Ok(Self::BlockMoveMoneyOutSetByParent),
            "card_usage_review_payout_status_hold" => Ok(Self::CardUsageReviewPayoutStatusHold),
            "no_available_balance" => Ok(Self::NoAvailableBalance),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMethodsResponseLimitsStandardErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountSuspended => write!(f, "account_suspended"),
            Self::BlockMoveMoneyOutBecauseClawback => {
                write!(f, "block_move_money_out_because_clawback")
            }
            Self::SupportabilityCheckPayoutStatusHold => {
                write!(f, "supportability_check_payout_status_hold")
            }
            Self::KycCompleted => write!(f, "kyc_completed"),
            Self::RmiClear => write!(f, "rmi_clear"),
            Self::IdentityRfiClear => write!(f, "identity_rfi_clear"),
            Self::EcommerceFulfillmentConnected => write!(f, "ecommerce_fulfillment_connected"),
            Self::BlockMoveMoneyOut => write!(f, "block_move_money_out"),
            Self::BlockMoveMoneyOutSetByParent => write!(f, "block_move_money_out_set_by_parent"),
            Self::CardUsageReviewPayoutStatusHold => {
                write!(f, "card_usage_review_payout_status_hold")
            }
            Self::NoAvailableBalance => write!(f, "no_available_balance"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
