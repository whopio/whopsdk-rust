pub use crate::prelude::*;

/// Reward recipient's role, or null for a fixed user who is not the requesting partner.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartnerReferralRewardRecipient {
    Business,
    Partner,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PartnerReferralRewardRecipient {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Business => serializer.serialize_str("business"),
            Self::Partner => serializer.serialize_str("partner"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PartnerReferralRewardRecipient {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "business" => Ok(Self::Business),
            "partner" => Ok(Self::Partner),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PartnerReferralRewardRecipient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Business => write!(f, "business"),
            Self::Partner => write!(f, "partner"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
