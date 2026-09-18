pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListPartnerReferralRequestsRequestRequestType {
    Manual,
    OwnershipTransfer,
    RewardLink,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListPartnerReferralRequestsRequestRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Manual => serializer.serialize_str("manual"),
            Self::OwnershipTransfer => serializer.serialize_str("ownership_transfer"),
            Self::RewardLink => serializer.serialize_str("reward_link"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListPartnerReferralRequestsRequestRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "manual" => Ok(Self::Manual),
            "ownership_transfer" => Ok(Self::OwnershipTransfer),
            "reward_link" => Ok(Self::RewardLink),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListPartnerReferralRequestsRequestRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manual => write!(f, "manual"),
            Self::OwnershipTransfer => write!(f, "ownership_transfer"),
            Self::RewardLink => write!(f, "reward_link"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
