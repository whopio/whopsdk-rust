pub use crate::prelude::*;

/// Whether the commission is a percentage or a flat fee.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipAffiliateCommissionType {
    FlatFee,
    Percentage,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipAffiliateCommissionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FlatFee => serializer.serialize_str("flat_fee"),
            Self::Percentage => serializer.serialize_str("percentage"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipAffiliateCommissionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "flat_fee" => Ok(Self::FlatFee),
            "percentage" => Ok(Self::Percentage),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipAffiliateCommissionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FlatFee => write!(f, "flat_fee"),
            Self::Percentage => write!(f, "percentage"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
