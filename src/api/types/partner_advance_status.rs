pub use crate::prelude::*;

/// The CPA payment lifecycle state.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartnerAdvanceStatus {
    Projected,
    Available,
    Accepted,
    Funded,
    ClawedBack,
    Completed,
    Expired,
    Withdrawn,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PartnerAdvanceStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Projected => serializer.serialize_str("projected"),
            Self::Available => serializer.serialize_str("available"),
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Funded => serializer.serialize_str("funded"),
            Self::ClawedBack => serializer.serialize_str("clawed_back"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::Withdrawn => serializer.serialize_str("withdrawn"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PartnerAdvanceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "projected" => Ok(Self::Projected),
            "available" => Ok(Self::Available),
            "accepted" => Ok(Self::Accepted),
            "funded" => Ok(Self::Funded),
            "clawed_back" => Ok(Self::ClawedBack),
            "completed" => Ok(Self::Completed),
            "expired" => Ok(Self::Expired),
            "withdrawn" => Ok(Self::Withdrawn),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PartnerAdvanceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Projected => write!(f, "projected"),
            Self::Available => write!(f, "available"),
            Self::Accepted => write!(f, "accepted"),
            Self::Funded => write!(f, "funded"),
            Self::ClawedBack => write!(f, "clawed_back"),
            Self::Completed => write!(f, "completed"),
            Self::Expired => write!(f, "expired"),
            Self::Withdrawn => write!(f, "withdrawn"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
