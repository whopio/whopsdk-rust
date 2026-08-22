pub use crate::prelude::*;

/// The calculation method for affiliate rev-share percentages
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AffiliateRevenueBases {
    PreFees,
    PostFees,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AffiliateRevenueBases {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PreFees => serializer.serialize_str("pre_fees"),
            Self::PostFees => serializer.serialize_str("post_fees"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AffiliateRevenueBases {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pre_fees" => Ok(Self::PreFees),
            "post_fees" => Ok(Self::PostFees),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AffiliateRevenueBases {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PreFees => write!(f, "pre_fees"),
            Self::PostFees => write!(f, "post_fees"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
