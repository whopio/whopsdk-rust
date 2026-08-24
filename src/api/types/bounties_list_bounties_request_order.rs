pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListBountiesRequestOrder {
    CreatedAt,
    GrossPaidOutAmount,
    GrossRewardAmount,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListBountiesRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::GrossPaidOutAmount => serializer.serialize_str("gross_paid_out_amount"),
            Self::GrossRewardAmount => serializer.serialize_str("gross_reward_amount"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListBountiesRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "gross_paid_out_amount" => Ok(Self::GrossPaidOutAmount),
            "gross_reward_amount" => Ok(Self::GrossRewardAmount),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListBountiesRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::GrossPaidOutAmount => write!(f, "gross_paid_out_amount"),
            Self::GrossRewardAmount => write!(f, "gross_reward_amount"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
