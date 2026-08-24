pub use crate::prelude::*;

/// Whether money moved and off whose balance: `none`, `merchant`, or `platform` (Whop refunded the customer and the merchant kept the funds). Independent of `outcome` — a case the merchant won can still carry a platform refund. `null` while the case is open, and on older closed cases that predate this being recorded.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionCenterCaseRefund {
    None,
    Merchant,
    Platform,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResolutionCenterCaseRefund {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Merchant => serializer.serialize_str("merchant"),
            Self::Platform => serializer.serialize_str("platform"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResolutionCenterCaseRefund {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "merchant" => Ok(Self::Merchant),
            "platform" => Ok(Self::Platform),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResolutionCenterCaseRefund {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Merchant => write!(f, "merchant"),
            Self::Platform => write!(f, "platform"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
