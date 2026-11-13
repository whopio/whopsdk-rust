pub use crate::prelude::*;

/// This account's reward state. Credited requires a posted ledger entry; processing includes a met requirement awaiting fulfillment. Reversing and reversed reflect a subsequent reward reversal.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountPartnerRewardStatus {
    InProgress,
    Processing,
    Credited,
    Reversing,
    Reversed,
    Unavailable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountPartnerRewardStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::InProgress => serializer.serialize_str("in_progress"),
            Self::Processing => serializer.serialize_str("processing"),
            Self::Credited => serializer.serialize_str("credited"),
            Self::Reversing => serializer.serialize_str("reversing"),
            Self::Reversed => serializer.serialize_str("reversed"),
            Self::Unavailable => serializer.serialize_str("unavailable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountPartnerRewardStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "in_progress" => Ok(Self::InProgress),
            "processing" => Ok(Self::Processing),
            "credited" => Ok(Self::Credited),
            "reversing" => Ok(Self::Reversing),
            "reversed" => Ok(Self::Reversed),
            "unavailable" => Ok(Self::Unavailable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountPartnerRewardStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InProgress => write!(f, "in_progress"),
            Self::Processing => write!(f, "processing"),
            Self::Credited => write!(f, "credited"),
            Self::Reversing => write!(f, "reversing"),
            Self::Reversed => write!(f, "reversed"),
            Self::Unavailable => write!(f, "unavailable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
