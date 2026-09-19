pub use crate::prelude::*;

/// Activity that qualifies for this reward, when specified.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartnerReferralRewardBotQualificationType {
    Sales,
    AdSpend,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PartnerReferralRewardBotQualificationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sales => serializer.serialize_str("sales"),
            Self::AdSpend => serializer.serialize_str("ad_spend"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PartnerReferralRewardBotQualificationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sales" => Ok(Self::Sales),
            "ad_spend" => Ok(Self::AdSpend),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PartnerReferralRewardBotQualificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sales => write!(f, "sales"),
            Self::AdSpend => write!(f, "ad_spend"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
