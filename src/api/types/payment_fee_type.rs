pub use crate::prelude::*;

/// The family the fee belongs to: `whop_fee`, `processing_fee`, `affiliate_program_fee`, or `other_fee`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentFeeType {
    WhopFee,
    ProcessingFee,
    AffiliateProgramFee,
    OtherFee,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentFeeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WhopFee => serializer.serialize_str("whop_fee"),
            Self::ProcessingFee => serializer.serialize_str("processing_fee"),
            Self::AffiliateProgramFee => serializer.serialize_str("affiliate_program_fee"),
            Self::OtherFee => serializer.serialize_str("other_fee"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentFeeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "whop_fee" => Ok(Self::WhopFee),
            "processing_fee" => Ok(Self::ProcessingFee),
            "affiliate_program_fee" => Ok(Self::AffiliateProgramFee),
            "other_fee" => Ok(Self::OtherFee),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentFeeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WhopFee => write!(f, "whop_fee"),
            Self::ProcessingFee => write!(f, "processing_fee"),
            Self::AffiliateProgramFee => write!(f, "affiliate_program_fee"),
            Self::OtherFee => write!(f, "other_fee"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
