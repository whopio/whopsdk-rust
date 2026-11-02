pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrieveBreakdownResponseItemsItemObject {
    User,
    Account,
    Merchant,
    PayoutDestination,
    Balance,
    WithdrawalAdjustment,
    AdCampaign,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrieveBreakdownResponseItemsItemObject {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::Account => serializer.serialize_str("account"),
            Self::Merchant => serializer.serialize_str("merchant"),
            Self::PayoutDestination => serializer.serialize_str("payout_destination"),
            Self::Balance => serializer.serialize_str("balance"),
            Self::WithdrawalAdjustment => serializer.serialize_str("withdrawal_adjustment"),
            Self::AdCampaign => serializer.serialize_str("ad_campaign"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrieveBreakdownResponseItemsItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user" => Ok(Self::User),
            "account" => Ok(Self::Account),
            "merchant" => Ok(Self::Merchant),
            "payout_destination" => Ok(Self::PayoutDestination),
            "balance" => Ok(Self::Balance),
            "withdrawal_adjustment" => Ok(Self::WithdrawalAdjustment),
            "ad_campaign" => Ok(Self::AdCampaign),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrieveBreakdownResponseItemsItemObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Account => write!(f, "account"),
            Self::Merchant => write!(f, "merchant"),
            Self::PayoutDestination => write!(f, "payout_destination"),
            Self::Balance => write!(f, "balance"),
            Self::WithdrawalAdjustment => write!(f, "withdrawal_adjustment"),
            Self::AdCampaign => write!(f, "ad_campaign"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
