pub use crate::prelude::*;

/// The link's lifecycle status. A newly funded link is `pending`. Check `claimable` for current availability, including expiry and reserved claims.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClaimClaimLinksResponseStatus {
    Pending,
    FullyClaimed,
    Canceled,
    Expired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ClaimClaimLinksResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::FullyClaimed => serializer.serialize_str("fully_claimed"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ClaimClaimLinksResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "fully_claimed" => Ok(Self::FullyClaimed),
            "canceled" => Ok(Self::Canceled),
            "expired" => Ok(Self::Expired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ClaimClaimLinksResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::FullyClaimed => write!(f, "fully_claimed"),
            Self::Canceled => write!(f, "canceled"),
            Self::Expired => write!(f, "expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
