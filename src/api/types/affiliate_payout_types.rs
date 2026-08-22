pub use crate::prelude::*;

/// The types of payouts an affiliate can have
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AffiliatePayoutTypes {
    Percentage,
    FlatFee,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AffiliatePayoutTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Percentage => serializer.serialize_str("percentage"),
            Self::FlatFee => serializer.serialize_str("flat_fee"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AffiliatePayoutTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "percentage" => Ok(Self::Percentage),
            "flat_fee" => Ok(Self::FlatFee),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AffiliatePayoutTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Percentage => write!(f, "percentage"),
            Self::FlatFee => write!(f, "flat_fee"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
